use crate::prebuilt::*;

/// The action of fetching attention list.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct FetchAttention;

impl Location<HoleSet> for FetchAttention {
    fn locate(&self, mut url: Url) -> Url {
        url.query_pairs_mut().append_pair("action", "getattention");
        url
    }

    fn dispatch(
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
    }
}
