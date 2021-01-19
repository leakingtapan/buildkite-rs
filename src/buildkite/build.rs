use crate::buildkite::types::{Build, Result};
use crate::buildkite::http::HttpClient;
use crate::buildkite::http;

pub struct BuildService<'a> {
    pub client: &'a HttpClient,
}

impl<'a> BuildService<'a> {
    pub fn new(client: &'a HttpClient) -> BuildService {
        BuildService {
            client: client,
        }
    }

    pub fn list_builds(&self, org: &str, pipeline: &str) -> Result<Vec<Build>> {
        let base_url = http::base_url(org);
        let url = format!("{}/pipelines/{}/builds", base_url, pipeline);
        self.client.get_response(url.as_str()) 
    }

}
