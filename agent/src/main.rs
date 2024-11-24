use clap::Parser;
use std::sync::Arc;
use tracing_subscriber::{fmt, EnvFilter};
use webterm_agent::args::Args;
use webterm_agent::config::Config;
use webterm_agent::models::panic_error::PanicError;
use webterm_agent::start;

#[tokio::main]
async fn main() -> Result<(), PanicError> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let args = Args::parse();
    let config = Arc::new(Config::new(args));
    start(config).await?;

    Ok(())
}
