use buildkite;
use crate::writer;
use clap::Clap;

/// ListPipelines show all the pipelines
#[derive(Clap)]
pub struct ListPipelines {
    /// Sets the pipeline name
    #[clap(short, long)]
    organization: String,
}

impl ListPipelines {
    pub fn run(&self, client: &buildkite::client::Client) {
        let pipelines = client.pipeline().list_pipelines(self.organization.as_str());
        writer::print_json(&pipelines);
    }
}

/// UpdatePipelines updates a given pipeline
#[derive(Clap)]
pub struct UpdatePipeline {}

/// GetPipeline get the pipeline for a given pipeline slug
#[derive(Clap)]
pub struct GetPipeline {
    /// Sets the pipeline name
    #[clap(short, long)]
    organization: String,

    /// Sets the pipeline slug
    #[clap(short, long)]
    slug: String,
}

impl GetPipeline {
    pub fn run(&self, client: &buildkite::client::Client) {
        let pipeline = client.pipeline().get_pipeline(self.organization.as_str(), self.slug.as_str());
        writer::print_json(&pipeline);
    }
}
