use clap::Parser;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};
use webterm_agent::args::Args;
use webterm_agent::config::Config;
use webterm_agent::daemonise::daemonise;
use webterm_agent::models::panic_error::PanicError;
use webterm_agent::start;

fn main() -> Result<(), PanicError> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();
    let args = Args::parse();
    let config = Arc::new(Config::new(args));

    if config.wants_daemon() {
        if !config.can_daemon() {
            return Err(PanicError::RuntimeError(
                "--daemon option is not supported on this platform".to_string(),
            ));
        } else {
            daemonise()?;
        }
    }

    tokio_main(config)
}

#[tokio::main]
async fn tokio_main(config: Arc<Config>) -> Result<(), PanicError> {
    info!("Starting webterm-agent…");
    start(config).await?;
    Ok(())
}
