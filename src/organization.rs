use crate::buildkite;
use crate::writer;
use clap::Clap;

/// GetOrganization get the organization given the name
#[derive(Clap)]
pub struct GetOrganization {
    /// Sets the pipeline name
    #[clap(short, long)]
    organization: String,
}

impl GetOrganization {
    pub fn run(&self, client: &buildkite::client::Client) {
        let organization = client.organization().get_organizatoin(self.organization.as_str());
        writer::print_json(&organization);
    }
}
