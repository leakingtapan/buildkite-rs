use crate::buildkite::types::{Pipeline, Result};
use crate::buildkite::http::HttpClient;
use crate::buildkite::http;

pub struct PipelineService<'a> {
    /// The buildkite client
    pub client: &'a HttpClient,
}

impl<'a> PipelineService<'a> {
    pub fn new(client: &'a HttpClient) -> PipelineService {
        PipelineService {
            client: client,
        }
    }

    /// List pipelines returns the pipeline list
    pub fn list_pipelines(&self, organization: &str) -> Result<Vec<Pipeline>> {
        let base_url = http::base_url(organization);
        let url = format!("{}/pipelines", base_url);
        self.client.get_response(url.as_str())
    }

    /// Get pipeline returns the pipeline for the given slug
    pub fn get_pipeline(&self, organization: &str, slug: &str) -> Result<Pipeline> {
        let base_url = http::base_url(organization);
        let url = format!("{}/pipelines/{}", base_url, slug);
        self.client.get_response(url.as_str())
    }
}
