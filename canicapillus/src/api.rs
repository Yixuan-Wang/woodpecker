use std::env;

use dotenv::dotenv;

use url::Url;

use crate::{
    common::{Endpoint, Location},
    HoleSet, ReplySet,
};

pub const DEFAULT_API_BASE: &str = "https://treehole.pku.edu.cn/api/";
const DEFAULT_PARAMS: [(&str, &str); 0] =
    [/* ("PKUHelperAPI", "3.0"), ("jsapiver", "201027113050-459074") */];

// A entrypoint
pub struct API {
    endpoint: Url,
    user_token: String,
}

impl Default for API {
    /// Returns a default API instance.
    ///
    /// # Panics
    ///
    /// Panics if `WOODPECKER_USER_TOKEN` is not found in env vars.
    fn default() -> Self {
        dotenv().ok();
        let user_token =
            env::var("WOODPECKER_USER_TOKEN").expect("WOODPECKER_USER_TOKEN not found.");
        let base = Url::parse_with_params(
            DEFAULT_API_BASE,
            DEFAULT_PARAMS
                .iter()
                // .chain([("user_token", user_token.as_str())].iter()),
        )
        .unwrap();

        API {
            endpoint: base,
            user_token,
        }
    }
}

impl API {
    /// Returns a new API instance.
    ///
    /// # Panics
    ///
    /// Panics if can not parse an absolute URL from `endpoint`.
    pub fn new(base: &str, params: Option<&[(&str, &str)]>, user_token: &str) -> Self {
        let base = Url::parse_with_params(
            base,
            params
                .into_iter()
                .flatten()
                // .chain([("user_token", user_token)].iter()),
        )
        .unwrap();

        API {
            endpoint: base,
            user_token: String::from(user_token),
        }
    }

    pub fn user_token(&self) -> &str {
        &self.user_token
    }
}

impl Endpoint<HoleSet> for API {
    fn locate(&self, location: &dyn Location<HoleSet>) -> Url {
        location.locate(self.endpoint.clone())
    }

    fn user_token(&self) -> &str {
        self.user_token()
    }
}

impl Endpoint<ReplySet> for API {
    fn locate(&self, location: &dyn Location<ReplySet>) -> Url {
        location.locate(self.endpoint.clone())
    }

    fn user_token(&self) -> &str {
        self.user_token()
    }
}