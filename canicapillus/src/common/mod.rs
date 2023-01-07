use thiserror::Error;
use url::Url;

mod resource;
pub use resource::*;

pub trait Endpoint<R>: Sized {
    fn locate(&self, location: &dyn Location<R>) -> Url;
    fn user_token(&self) -> &str;
}

#[derive(Error, Debug)]
pub enum SwarmError {
    #[error("Swarm not supported.")]
    Unsupported,
    #[cfg(feature = "fireman")]
    #[error("")]
    Fireman,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Swarm {
    Concurrent { count: usize, page_size: usize },
    Sequential { count: usize, page_size: usize },
}
