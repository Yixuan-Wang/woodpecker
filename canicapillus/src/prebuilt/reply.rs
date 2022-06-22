use super::*;

/// The action of fetching attention list.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct FetchReply { 
    pub hole_id: hole::HoleID
}

impl Location<ReplySet> for FetchReply {
    fn locate(&self, mut url: Url) -> Url {
        url.query_pairs_mut()
            .extend_pairs([("action", "getcomment"), ("pid", &String::from(self.hole_id))]);
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
