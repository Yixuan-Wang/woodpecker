use std::{marker::PhantomData, sync::Arc};

use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::{self, header, Client};
use thiserror::Error;
use tokio::{
    join,
    sync::{mpsc, Mutex as AsyncMutex},
};
use url::{ParseError as UrlParseError, Url};

use crate::{
    api::API,
    common::{
        Endpoint, Location, MergeResourceError, ParseResourceError, Resource, Swarm, SwarmError,
    },
};

#[derive(Error, Debug)]
pub enum FetcherError {
    // /// Cannot find user token in env vars.
    // #[error("WOODPECKER_USER_TOKEN is not found.")]
    // UserTokenNotFound,
    /// Failed to build a [`reqwest::Client`].
    #[error("Fails to build a request client.")]
    ClientBuildFail(#[from] reqwest::Error),

    #[error("Malformed url.")]
    MalformedUrl(#[from] UrlParseError),

    #[error("Swarm fails.")]
    SwarmFail(#[from] SwarmError),

    #[error("Client pool fails.")]
    ClientPoolFail,

    #[error("Fails to parse a resource.")]
    ParseResourceFail(#[from] ParseResourceError),

    #[error("Fails to merge two resources.")]
    MergeResourceFail(#[from] MergeResourceError),
}

pub trait FetcherClientBuilder {
    fn build(&self) -> Client;
}

pub struct DefaultFetcherClientBuilder;

impl FetcherClientBuilder for DefaultFetcherClientBuilder {
    fn build(&self) -> Client {
        let mut default_headers = header::HeaderMap::new();
        default_headers.insert(
            header::REFERER,
            header::HeaderValue::from_static("https://pkuhelper.pku.edu.cn/hole/"),
        );
        default_headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36"));

        Client::builder()
            .default_headers(default_headers)
            .build()
            .unwrap()
    }
}

struct FetcherClientNode<R>
where
    R: Resource,
{
    client: Client,
    rx: Arc<AsyncMutex<mpsc::Receiver<Option<Url>>>>,
    ret: mpsc::Sender<Result<R, FetcherError>>,
}

impl<R> FetcherClientNode<R>
where
    R: Resource,
{
    async fn work(&mut self) -> Result<(), FetcherError> {
        loop {
            let recv = self.rx.lock().await.recv().await;
            match recv {
                Some(Some(url)) => {
                    let result = self.fetch(url).await;
                    self.ret
                        .send(result)
                        .await
                        .or(Err(FetcherError::ClientPoolFail))?;
                }
                _ => {
                    self.rx.lock().await.close();
                    break;
                }
            }
        }
        Ok(())
    }

    #[inline]
    async fn fetch(&self, url: Url) -> Result<R, FetcherError> {
        let res = self.client.get(url).send().await?;
        Ok(R::parse(res).await?)
    }
}

/// A [`Fetcher`] is a client requesting the hole backend API.
pub struct Fetcher<A, R>
where
    A: Endpoint<R>,
{
    /// A hook that specifies how to build a [`reqwest::Client`] internally.
    client_builder: Box<dyn FetcherClientBuilder>,
    /// The hole backend API.
    api: A,
    phantom: PhantomData<R>,
}

pub struct FetcherExecutor<'fch, 'lct, A, R>
where
    A: Endpoint<R>,
    R: Resource,
{
    fetcher: &'fch mut Fetcher<A, R>,
    location: &'lct dyn Location<R>,
    swarm: Option<Swarm>,
}

macro_rules! must_be {
    ($p:pat = $e:expr => $r: expr) => {{
        if let $p = $e {
            $r
        } else {
            unreachable!()
        }
    }};
    ($e:expr, $p:pat, $r: expr) => {{
        if let $p = $e {
            $r
        } else {
            unreachable!()
        }
    }};
}

impl<'fch, 'lct, A, R> FetcherExecutor<'fch, 'lct, A, R>
where
    A: Endpoint<R>,
    R: Resource,
{
    pub async fn execute(&mut self) -> Result<R, FetcherError> {
        match self.swarm {
            None => self.execute_one().await,
            Some(Swarm::Concurrent { .. }) => self.execute_parallel().await,
            Some(Swarm::Sequential { .. }) => self.execute_sequential().await,
        }
    }

    pub fn swarm(mut self, swarm: Option<Swarm>) -> Self {
        self.swarm = swarm;
        self
    }

    async fn execute_one(&mut self) -> Result<R, FetcherError> {
        const MEANINGLESS: usize = 1;
        let client = self.fetcher.client_builder.build();
        let url = self.fetcher.api.locate(self.location);
        let url = self
            .location
            .dispatch(url, None, MEANINGLESS, MEANINGLESS)?;
        let res = client.get(url).send().await?;

        Ok(R::parse(res).await?)
    }

    async fn execute_sequential(&mut self) -> Result<R, FetcherError> {
        let (swarm, count, page_size) = must_be!(
            self.swarm,
            Some(ref swarm @ Swarm::Sequential { count, page_size }),
            (swarm, count, page_size)
        );
        let client = self.fetcher.client_builder.build();
        let mut results = R::default();
        for page in 1..=count {
            let url = self.fetcher.api.locate(self.location);
            let url = self.location.dispatch(url, Some(swarm), page, page_size)?;
            let res = client.get(url).send().await;
            if let Ok(res) = res {
                let one_result = R::parse(res).await;
                if let Ok(result) = one_result {
                    results = R::merge(results, result)?
                }
            }
        }

        Ok(results)
    }

    async fn execute_parallel(&mut self) -> Result<R, FetcherError> {
        let pool_size: usize = must_be!(
            Some(Swarm::Concurrent { count, .. }) = self.swarm => count.min(16)
        );

        // init workload channels
        let (tx, rx) = mpsc::channel(pool_size);
        let rx = Arc::new(AsyncMutex::new(rx));

        // init result channels
        let (ret, mut res) = mpsc::channel(pool_size);

        // initializing clients
        let mut clients = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            let client = self.fetcher.client_builder.build();
            let client = FetcherClientNode {
                client,
                rx: Arc::clone(&rx),
                ret: ret.clone(),
            };
            clients.push(client);
        }

        // launch works on clients, return an await handle
        let work = clients
            .iter_mut()
            .map(|client| async move { client.work().await })
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<_>>();

        // dispatch works for clients and collect results, return an await handle
        let results = async {
            let (swarm, count, page_size) = must_be! {
                Some(ref swarm @ Swarm::Concurrent { count, page_size }) = self.swarm => (swarm, count, page_size)
            };
            let dispatch = async {
                let dispatch_errors = (1..=count)
                    .into_iter()
                    .filter_map(|page| {
                        self.location
                            .dispatch(
                                self.fetcher.api.locate(self.location),
                                Some(swarm),
                                page,
                                page_size,
                            )
                            .ok()
                    })
                    .map(|url| {
                        dbg!(url.to_string());
                        let tx = tx.clone();
                        async move {
                            tx.send(Some(url))
                                .await
                                .or(Err(FetcherError::ClientPoolFail))
                        }
                    })
                    .collect::<FuturesUnordered<_>>()
                    .collect::<Vec<_>>()
                    .await;

                for _ in 0..pool_size {
                    tx.send(None).await.unwrap();
                }
                dispatch_errors
            };

            let collect = async {
                let mut result = R::default();
                for _ in 0..count {
                    if let Some(received) = res.recv().await {
                        match received {
                            Ok(new) => result = R::merge(result, new).unwrap(),
                            Err(e) => { dbg!(e); }
                        }
                    }
                }
                result
            };

            let (_, collect) = join!(dispatch, collect);
            collect
        };

        let (_, results) = join!(work, results);
        Ok(results)
    }
}

impl<A, R> Fetcher<A, R>
where
    A: Endpoint<R>,
    R: Resource,
{
    pub fn fetch<'fch, 'lct>(
        &'fch mut self,
        resource: &'lct dyn Location<R>,
    ) -> FetcherExecutor<'fch, 'lct, A, R> {
        let swarm = resource.default_swarm();
        FetcherExecutor {
            fetcher: self,
            location: resource,
            swarm,
        }
    }
}

/* async fn fetch_iterative(client: &Client, url: Url, flag: HoleFlag) -> Result<HoleSet, FetcherError> {
    let mut page: usize = 1;
    let mut count: usize = 0;
    while count < max {

    }
}
 */
impl<R> Default for Fetcher<API, R>
where
    API: Endpoint<R>,
{
    /// Try to return a new default [`Fetcher`] instance.
    ///
    /// This default instance uses:
    ///
    /// - Default [`API`] instance
    ///     - base URL: `https://pkuhelper.pku.edu.cn/services/pkuhole/api.php`
    ///     - token: `WOODPECKER_USER_TOKEN` provided in env vars
    /// - A [`reqwest::Client`] instance with
    ///     - referer: `https://pkuhelper.pku.edu.cn/hole/`
    ///     - user agent: `Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36`
    ///
    fn default() -> Self {
        Fetcher {
            client_builder: Box::new(DefaultFetcherClientBuilder),
            api: API::default(),
            phantom: PhantomData,
        }
    }
}
