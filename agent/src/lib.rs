pub mod args;
pub mod config;
pub mod messaging;
pub mod models;

use crate::config::Config;
use crate::models::agent_error::AgentError;
use crate::models::runner::Runner;

pub async fn start(config: Config) -> Result<(), AgentError> {
    let runner = Runner::new();
    runner.run(config).await
}
