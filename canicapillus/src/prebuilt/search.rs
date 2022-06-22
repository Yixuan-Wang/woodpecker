use super::*;

/// The action of fetching a search result.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct FetchSearch {
    pub keyword: String,
}

impl Location<HoleSet> for FetchSearch {
    fn locate(&self, mut url: Url) -> Url {
        url.query_pairs_mut()
            .extend_pairs([("action", "search"), ("keywords", &self.keyword)]);
        url
    }

    /// # Notes
    ///
    /// The default page size might be 10 on some hole endpoints.
    /// Requesting a page size over 50 is undefined behaviour.
    ///
    fn dispatch(
        &self,
        mut url: Url,
        swarm: Option<&Swarm>,
        page: usize,
        page_size: usize,
    ) -> Result<Url, crate::common::SwarmError> {
        if swarm.is_none() {
            url.query_pairs_mut()
                .extend_pairs([("pagesize", "50"), ("page", "1")]);
            return Ok(url);
        }

        #[cfg(feature = "fireman")]
        {
            if page_size > 50 || page > 3 {
                return Err(SwarmError::Fireman);
            }
        }
        assert!(page >= 1, "Resource {:?} requries page >= 1", self);

        url.query_pairs_mut().extend_pairs([
            ("pagesize", &page_size.to_string()),
            ("page", &page.to_string()),
        ]);
        Ok(url)
    }

    fn default_swarm(&self) -> Option<Swarm> {
        Some(Swarm::Concurrent {
            count: 3,
            page_size: 50,
        })
    }
}
