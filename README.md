# A Rust library for Buildkite API
A Rust binding to the Buildkite V2 API.

## Getting Started
First, add following to `Cargo.toml`:
```toml
[dependencies]
buildkite = "0.1.0"
```
Then use the crate with:
```rust
use buildkite;

fn main() {
    let client = buildkite::client::Client::new("BUILDKITE_TOKEN");
}
```

## Examples
### Organizations
```rust
// List Organizations
client
    .organizations()
    .list();

// Get a sepcific organization
client
    .organizations()
    .get(org_name);
```

### Builds
```rust
// List builds for a specific pipeline
client
    .builds()
    .list(org_name, pipeline_name);
```

### Agents
```rust
// List agents
client
    .agents()
    .list(org_name)

// Get agent
client
    .agents()
    .get(org_name, agent_id)

```

## License
Licensed under Apache License, Version 2.0

## TODOs
- [ ]  list for paginated results
- [ ]  custom buildkite error
- [ ]  add user agent
- [ ]  add marco to reduce boilerplated
- [ ]  add unit tests
- [ ]  add cancellation for each API
