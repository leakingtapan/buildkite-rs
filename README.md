# A Rust library for Buildkite API

# TODOs
- [  ] get paginated results
- [  ] create builkite subcommand
- [  ] create sub cargo module
- [  ] custom buildkite error
- [  ] builder pattern for command chaining
- [  ] add user agent

# Examples
```rust
let client = buildkite::Client::new("BUILDKITE_TOKEN")
// List Organizations
client
    .organizations()
    .list();

// Get a sepcific organization
client
    .organizations()
    .get(ctx, org_name);

// List builds for a specific pipeline
client
    .builds()
    .list(ctx, org_name, pipeline_name, opts);

// Get a specific build for a specific pipeline
client
    .builds()
    .get(ctx, org_name, pipeline_name, build_id, opts);
```
