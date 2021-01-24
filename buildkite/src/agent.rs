use crate::http;
use crate::http::HttpClient;
use crate::types::{Agent, Result};
use serde::{Deserialize, Serialize};

pub struct AgentService<'a> {
    pub client: &'a HttpClient,
}

impl<'a> AgentService<'a> {
    pub fn new(client: &'a HttpClient) -> AgentService<'a> {
        AgentService { client: client }
    }

    pub fn list(&self, org: &str) -> Result<Vec<Agent>> {
        let base_url = http::org_url(org);
        let url = format!("{}/agents", base_url);
        self.client.get_response(url.as_str())
    }

    pub fn get(&self, org: &str, agent_id: &str) -> Result<Vec<Agent>> {
        let base_url = http::org_url(org);
        let url = format!("{}/agents/{}", base_url, agent_id);
        self.client.get_response(url.as_str())
    }

    pub fn stop(&self, org: &str, agent_id: &str, force: bool) -> Result<()> {
        let base_url = http::org_url(org);
        let url = format!("{}/agents/{}", base_url, agent_id);
        let request = StopAgentRequest{force: force};
        self.client.put(url.as_str(), &request)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct StopAgentRequest {
    force: bool,
}
