use crate::writer;
use buildkite;
use clap::Clap;

/// gets the organization given the name
#[derive(Clap)]
pub struct GetOrganization {
    /// Sets the pipeline name
    #[clap(short, long)]
    organization: String,
}

impl GetOrganization {
    pub fn run(&self, client: &buildkite::client::Client) {
        let organization = client.organization().get(self.organization.as_str());
        writer::print_json(&organization);
    }
}

/// lists all the organizations
#[derive(Clap)]
pub struct ListOrganizations {
}

impl ListOrganizations {
    pub fn run(&self, client: &buildkite::client::Client) {
        let organizations = client.organization().list();
        writer::print_json(&organizations);
    }
}
