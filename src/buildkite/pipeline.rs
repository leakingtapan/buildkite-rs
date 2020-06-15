use crate::buildkite::types::{Pipeline, Result};
use crate::buildkite::Client;

impl Client {
    /// List pipelines returns the pipeline list
    pub fn list_pipelines(&self, organization: &str) -> Result<Vec<Pipeline>> {
        let base_url = self.base_url(organization);
        let url = format!("{}/pipelines", base_url);
        self.get_response(url.as_str())
    }

    /// Get pipeline returns the pipeline for the given slug
    pub fn get_pipeline(&self, organization: &str, slug: &str) -> Result<Pipeline> {
        let base_url = self.base_url(organization);
        let url = format!("{}/pipelines/{}", base_url, slug);
        self.get_response(url.as_str())
    }
}
