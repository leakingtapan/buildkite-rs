use reqwest::blocking;
use serde::de::DeserializeOwned;
use crate::types::Result;

// HttpClient is for handling http requests to Buildkite API
pub struct HttpClient {

    // internal http client
    client: blocking::Client,

    // buildkite API token
    token: String
}

impl HttpClient {
    pub fn new(token: String) -> HttpClient {
        HttpClient {
            client: blocking::Client::new(),
            token: token
        }
    }

    /// generic function to fetch the response and deserialize to struct of given type
    pub fn get_response<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        self.client
            .get(url)
            .bearer_auth(self.token.as_str())
            .send()?
            .json::<T>()
    }
}

pub const BUILDKITE_URL_BASE: &str = "https://api.buildkite.com/v2";

pub fn base_url() -> String {
    format!("{}", BUILDKITE_URL_BASE)
}
pub fn org_url(org: &str) -> String {
    format!("{}/organizations/{}", base_url(), org)
}
