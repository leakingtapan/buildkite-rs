use crate::buildkite::types::{Build, Result};
use crate::buildkite::Client;

impl Client {
    pub fn list_builds(&self, organization: &str, pipeline: &str) -> Result<Vec<Build>> {
        let base_url = self.base_url(organization);
        let url = format!("{}/pipelines/{}/builds", base_url, pipeline);
        self.get_response(url.as_str()) 
    }
}
