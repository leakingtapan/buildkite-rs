use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    started_at: Option<DateTime<Utc>>,
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
    url: String,
    web_url: String,
    name: String,
    connection_state: String,
    ip_address: String,
    hostname: String,
    user_agent: String,
    version: String,
    creator: Option<String>,
    created_at: DateTime<Utc>,
    job: Option<Job>,
    last_job_finished_at: Option<DateTime<Utc>>,
    priority: u32,
    meta_data: Vec<String>,
}

/// A `Result` alias where the `Err` case is `reqwest::Error`
pub type Result<T> = std::result::Result<T, reqwest::Error>;
