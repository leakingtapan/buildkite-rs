use buildkite;
use crate::writer;
use clap::Clap;

/// lists agents for a given organization
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

/// get agent for a given organization and agent ID
#[derive(Clap)]
pub struct GetAgent {
    /// Sets the organization name
    #[clap(short, long)]
    organization: String,

    /// Sets the agent ID
    #[clap(short, long)]
    agent_id: String,
}

impl GetAgent {
    pub fn run(&self, client: &buildkite::client::Client) {
        let agents = client
            .agent()
            .get(self.organization.as_str(), self.agent_id.as_str());
        writer::print_json(&agents);
    }
}
