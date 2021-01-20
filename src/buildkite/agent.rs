use crate::buildkite::types::{Agent, Result};
use crate::buildkite::http::HttpClient;
use crate::buildkite::http;

pub struct AgentService<'a> {
    pub client: &'a HttpClient,
}

impl<'a> AgentService<'a> {
    pub fn new(client: &'a HttpClient) -> AgentService<'a> {
        AgentService {
            client: client,
        }
    }

    pub fn list(&self, org: &str) -> Result<Vec<Agent>> {
        let base_url = http::org_url(org);
        let url = format!("{}/agents", base_url);
        self.client.get_response(url.as_str())
    }

}
