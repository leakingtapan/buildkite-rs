// This example demonstrates clap's full 'custom derive' style of creating arguments which is the
// simplest method of use, but sacrifices some flexibility.
use clap::Clap;

mod agent;
mod build;
mod buildkite;
mod organization;
mod pipeline;
mod writer;

#[derive(Clap)]
#[clap(version = "0.1", author = "pancheng1987@gmail.com")]
struct Opts {
    /// Sets the buildkite token
    #[clap(short, long)]
    token: String,

    /// The subcommand
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    ListPipelines(pipeline::ListPipelines),

    UpdatePipeline(pipeline::UpdatePipeline),

    GetPipeline(pipeline::GetPipeline),

    ListBuilds(build::ListBuilds),

    GetOrganization(organization::GetOrganization),

    ListAgents(agent::ListAgents),
}

impl SubCommand {
    fn run(&self, client: &buildkite::client::Client) {
        match self {
            SubCommand::ListPipelines(c) => c.run(&client),
            SubCommand::UpdatePipeline(_) => {}
            SubCommand::ListBuilds(c) => c.run(&client),
            SubCommand::GetOrganization(c) => c.run(&client),
            SubCommand::GetPipeline(c) => c.run(&client),
            SubCommand::ListAgents(c) => c.run(&client),
        }
    }
}

fn main() {
    let opts = Opts::parse();
    let client = buildkite::client::Client::new(opts.token.as_str());

    opts.subcmd.run(&client);
}
