use crate::buildkite::types::{Organization, Result};
use crate::buildkite::Client;

impl Client {
    pub fn get_organizatoin(&self, organization: &str) -> Result<Organization> {
        let base_url = self.base_url(organization);
        let url = format!("{}", base_url);
        self.get_response(url.as_str()) 
    }
}
