pub mod agent_version;
pub mod args;
pub mod config;
pub mod daemonise;
pub mod messaging;
pub mod models;

use crate::config::Config;
use crate::models::panic_error::PanicError;
use crate::models::runner::Runner;
use std::sync::Arc;

pub async fn start(config: Arc<Config>) -> Result<(), PanicError> {
    let runner = Runner::new();
    runner.run(config).await
}
