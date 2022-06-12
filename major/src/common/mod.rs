use async_trait::async_trait;
use reqwest::Response;
use thiserror::Error;
use url::Url;

pub trait Endpoint<R>: Sized {
    fn locate(&self, location: &dyn Location<R>) -> Url;
}

#[derive(Error, Debug)]
pub enum SwarmError {
    #[error("Swarm not supported.")]
    Unsupported,
    #[cfg(feature = "fireman")]
    #[error("")]
    Fireman,
}

pub trait Resource
where
    Self: Sized + ParseResource<Self> + MergeResource<Self>,
{
    type Specifier: Clone + Default;

    fn blank(flag: Self::Specifier) -> Self;
}

/// A resource which can be found on the specific endpoint `E`.
pub trait Location<R>
where
    Self: Sync,
    R: Resource,
{
    /// The location of the resource.
    fn locate(&self, url: Url) -> Url;

    /// A swarm location of the resource.
    fn dispatch(
        &self,
        url: Url,
        swarm: Option<&Swarm>,
        page: usize,
        page_size: usize,
    ) -> Result<Url, SwarmError>;

    /// The default swarm.
    fn default_swarm(&self) -> Option<Swarm> {
        None
    }
}

#[derive(Error, Debug)]
#[error("Parse resource failed.")]
pub struct ParseResourceError;

impl From<reqwest::Error> for ParseResourceError {
    fn from(_: reqwest::Error) -> Self {
        Self
    }
}

#[async_trait]
pub trait ParseResource<R>
where
    R: Resource,
{
    async fn parse(response: Response, flag: R::Specifier) -> Result<R, ParseResourceError>;
}

#[derive(Error, Debug)]
#[error("Merge resource failed.")]
pub struct MergeResourceError;

pub trait MergeResource<R>
where
    R: Resource,
{
    fn merge(lhs: R, rhs: R, flag: R::Specifier) -> Result<R, MergeResourceError>;
}

/* pub trait MergeResource<R, E>
where Self: Location<R> {
    fn merge(&self, lhs: R, rhs: R) -> Result<R, E>;
} */

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Swarm {
    Concurrent { count: usize, page_size: usize },
    Sequential { count: usize, page_size: usize },
}
