use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};
use webterm_agent::args::Args;
use webterm_agent::config::Config;
use webterm_agent::models::agent_error::AgentError;
use webterm_agent::start;

#[tokio::main]
async fn main() -> Result<(), AgentError> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let args = Args::parse();
    let config = Config::new(args);
    start(config).await?;

    Ok(())
}
