use crate::buildkite::agent::AgentService;
use crate::buildkite::build::BuildService;
use crate::buildkite::http::HttpClient;
use crate::buildkite::organization::OrganizationService;
use crate::buildkite::pipeline::PipelineService;

/// Client is the Buildkite API client
pub struct Client {
    /// The buildkite client
    client: HttpClient,
}

impl Client {
    /// new returns a new buildkite client
    pub fn new(token: &str) -> Self {
        Client {
            client: HttpClient::new(token.to_string()),
        }
    }

    pub fn organization<'a>(&'a self) -> OrganizationService {
        OrganizationService::new(&self.client)
    }

    pub fn agent<'a>(&'a self) -> AgentService {
        AgentService::new(&self.client)
    }

    pub fn build<'a>(&'a self) -> BuildService {
        BuildService::new(&self.client)
    }

    pub fn pipeline<'a>(&'a self) -> PipelineService {
        PipelineService::new(&self.client)
    }
}
