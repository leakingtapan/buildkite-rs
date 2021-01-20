use buildkite;
use crate::writer;
use clap::Clap;

/// ListBuilds lists builds for a given pipeline
#[derive(Clap)]
pub struct ListAgents {
    /// Sets the organization name
    #[clap(short, long)]
    organization: String,
}

impl ListAgents {
    pub fn run(&self, client: &buildkite::client::Client) {
        let agents = client
            .agent()
            .list(self.organization.as_str());
        writer::print_json(&agents);
    }
}
