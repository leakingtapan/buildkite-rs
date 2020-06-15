use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    name: String,
    command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline {
    slug: String,
    description: Option<String>,
    configuration: Option<String>,
    steps: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    id: String,
    state: String,
    jobs: Vec<Job>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    id: String,
    name: String,
    started_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    id: String,
    slug: String,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    id: String,
    name: String,
    connection_state: String,
    ip_address: String,
    hostname: String,
}

/// A `Result` alias where the `Err` case is `reqwest::Error`
pub type Result<T> = std::result::Result<T, reqwest::Error>;
