use async_trait::async_trait;
use reqwest::Response;
use thiserror::Error;
use url::Url;

use crate::{HoleSet, ReplySet, hole::{RawHolePage, reply::RawReplyPage}};

use super::{Swarm, SwarmError};

pub trait Resource
where
    Self: Sized + std::fmt::Debug + Default + ParseResource<Self> + MergeResource<Self>,
{}

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
    async fn parse(response: Response) -> Result<R, ParseResourceError>;
}

#[derive(Error, Debug)]
#[error("Merge resource failed.")]
pub struct MergeResourceError;

pub trait MergeResource<R>
where
    R: Resource,
{
    fn merge(lhs: R, rhs: R) -> Result<R, MergeResourceError>;
}

#[macro_export]
macro_rules! derive_resource_set {
    ($the_field:ident, $the_page:tt, $the_set:tt) => {
        impl Resource for $the_set {}
        
        #[async_trait]
        impl ParseResource<$the_set> for $the_set {
            async fn parse(
                response: reqwest::Response,
            ) -> Result<$the_set, ParseResourceError> {
                let page: Result<$the_page, _> = response.json().await;
                match page {
                    Ok(p) => Ok(p.into_iter().collect()),
                    Err(e) => {
                        dbg!(&e);
                        Err(ParseResourceError::from(e))
                    }
                }
            }
        }
        
        impl MergeResource<$the_set> for $the_set {
            fn merge(
                mut lhs: $the_set,
                rhs: $the_set,
            ) -> Result<$the_set, crate::common::MergeResourceError> {
                lhs.extend(rhs);
                Ok(lhs)
            }
        }
    };
}

derive_resource_set! {
    hole, RawHolePage, HoleSet
}

derive_resource_set! {
    reply, RawReplyPage, ReplySet
}
