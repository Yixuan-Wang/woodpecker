use crate::prebuilt::*;

/// The action of fetching live feed.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct FetchFeed;

impl Location<HoleSet> for FetchFeed {
    fn locate(&self, mut url: Url) -> Url {
        url.query_pairs_mut().append_pair("action", "getlist");
        url
    }

    fn dispatch(
        &self,
        mut url: Url,
        swarm: Option<&Swarm>,
        page: usize,
        _page_size: usize,
    ) -> Result<Url, crate::common::SwarmError> {
        if swarm.is_none() {
            url.query_pairs_mut().append_pair("p", "1");
            return Ok(url);
        }

        #[cfg(feature = "fireman")]
        {
            if page > 100 {
                return Err(SwarmError::Fireman);
            }
        }
        assert!(page >= 1, "Resource {:?} requires page >= 1", self);

        url.query_pairs_mut().append_pair("p", &page.to_string());
        Ok(url)
    }

    fn default_swarm(&self) -> Option<Swarm> {
        Some(Swarm::Concurrent {
            count: 4,
            page_size: 30,
        })
    }
}
