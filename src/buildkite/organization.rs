use crate::buildkite::types::{Organization, Result};
use crate::buildkite::http::HttpClient;
use crate::buildkite::http;

pub struct OrganizationService<'a> {
    /// The buildkite client
    pub client: &'a HttpClient,
}

impl<'a> OrganizationService<'a> {
    pub fn new(client: &'a HttpClient) -> OrganizationService {
        OrganizationService {
            client: client,
        }
    }

    pub fn list(&self) -> Result<Vec<Organization>> {
        let base_url = http::base_url();
        let url = format!("{}", base_url);
        self.client.get_response(url.as_str()) 
    }

    pub fn get(&self, org: &str) -> Result<Organization> {
        let base_url = http::org_url(org);
        let url = format!("{}", base_url);
        self.client.get_response(url.as_str())
    }

}