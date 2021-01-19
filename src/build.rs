use crate::buildkite;
use crate::writer;
use clap::Clap;

/// ListBuilds lists builds for a given pipeline
#[derive(Clap)]
pub struct ListBuilds {
    /// Sets the pipeline name
    #[clap(short, long)]
    organization: String,

    /// Sets the pipeline name
    #[clap(short, long)]
    pipeline: String,
}

impl ListBuilds {
    pub fn run(&self, client: &buildkite::client::Client) {
        let builds = client.build().list_builds(self.organization.as_str(), self.pipeline.as_str());
        writer::print_json(&builds);
    }
}
