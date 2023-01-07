use super::*;

/// The action of fetching attention list.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct FetchReply { 
    pub hole_id: hole::HoleID
}

impl Location<ReplySet> for FetchReply {
    fn locate(&self, url: Url) -> Url {
        let url = url.join(&format!("pku_comment/{}", String::from(self.hole_id))).unwrap();
        url
    }

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
}
