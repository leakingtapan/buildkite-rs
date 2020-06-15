use crate::buildkite::types::{Agent, Result};
use crate::buildkite::Client;

pub struct AgentService<'a> {
    client: &'a Client,

    organization: &'a str,
}

impl<'a> AgentService<'a> {
    pub fn new(client: &'a Client, organization: &'a str) -> AgentService<'a> {
        AgentService {
            client: client,
            organization: organization,
        }
    }
    pub fn list(&self) -> Result<Vec<Agent>> {
        let base_url = self.client.base_url(self.organization);
        let url = format!("{}/agents", base_url);
        self.client.get_response(url.as_str())
    }
}
