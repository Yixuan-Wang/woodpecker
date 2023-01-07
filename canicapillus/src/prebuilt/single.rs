use super::*;

/// The action of fetching a single hole.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct FetchSingle {
    pub id: hole::HoleID,
}

impl Location<HoleSet> for FetchSingle {
    fn locate(&self, url: Url) -> Url {
        let url = url.join(&format!("pku/{}", String::from(self.id))).unwrap();
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
