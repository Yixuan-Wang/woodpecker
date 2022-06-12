use reqwest::Url;

use crate::{
    common::{Location, Swarm, SwarmError},
    hole::*,
};

mod attention;
mod feed;
mod search;
mod single;

pub use {attention::FetchAttention, feed::FetchFeed, search::FetchSearch, single::FetchSingle};
