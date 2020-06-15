use crate::buildkite;
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
    pub fn run(&self, client: &buildkite::Client) {
        //let agents = client.list_agents(self.organization.as_str());
        let agents = client
            .organization(self.organization.as_str())
            .agent()
            .list();
        writer::print_json(&agents);
    }
}
