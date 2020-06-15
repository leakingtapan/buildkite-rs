use reqwest::blocking;
use serde::de::DeserializeOwned;

use crate::buildkite::types::Result;
use crate::buildkite::agent::AgentService;

mod agent;
mod build;
mod client;
mod organization;
mod pipeline;
mod types;

/// Client is the Buildkite API client
pub struct Client {
    /// The buildkite client
    client: blocking::Client,

    /// The auth token
    token: String,
}

impl Client {
    /// new returns a new buildkite client
    pub fn new(token: &str) -> Self {
        Client {
            client: blocking::Client::new(),
            token: token.to_string(),
        }
    }

    pub fn organization<'a>(&'a self, organization: &'a str) -> OrganizationClient {
        OrganizationClient{
            client: &self,
            organization: organization,
        }
    }

    /// generic function to fetch the response and deserialize to struct of given type
    fn get_response<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        self.client
            .get(url)
            .bearer_auth(self.token.as_str())
            .send()?
            .json::<T>()
    }

    /// get the base URL
    fn base_url(&self, organization: &str) -> String {
        format!(
            "https://api.buildkite.com/v2/organizations/{}/",
            organization
        )
    }
    
}

pub struct OrganizationClient<'a> {
    /// The buildkite client
    client: &'a Client,

    /// The organization name
    organization: &'a str,
}

impl<'a> OrganizationClient<'a> {
    pub fn agent(&self) -> AgentService {
        AgentService::new(self.client, self.organization)
    }
}
