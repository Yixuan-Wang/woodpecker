use super::*;

/// The action of fetching attention list.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct FetchAttention;

impl Location<HoleSet> for FetchAttention {
    fn locate(&self, url: Url) -> Url {
        let url = url.join("follow").unwrap();
        url
    }

    /* fn dispatch(
        &self,
        url: Url,
        swarm: Option<&Swarm>,
        _page: usize,
        _page_size: usize,
    ) -> Result<Url, crate::common::SwarmError> {
        if swarm.is_none() {
            Ok(url)
        } else {
            Err(SwarmError::Unsupported)
        }
    } */

    fn dispatch(
        &self,
        mut url: Url,
        swarm: Option<&Swarm>,
        page: usize,
        page_size: usize,
    ) -> Result<Url, crate::common::SwarmError> {
        if swarm.is_none() {
            url.query_pairs_mut().extend_pairs([("page", "1"), ("limit", "25")]);
            return Ok(url);
        }

        #[cfg(feature = "fireman")]
        {
            if page > 100 {
                return Err(SwarmError::Fireman);
            }
        }
        assert!(page >= 1, "Resource {:?} requires page >= 1", self);

        url.query_pairs_mut().extend_pairs([("page", &page.to_string()), ("limit", &page_size.to_string())]);
        Ok(url)
    }

    fn default_swarm(&self) -> Option<Swarm> {
        Some(Swarm::Concurrent {
            count: 4,
            page_size: 30,
        })
    }
}
