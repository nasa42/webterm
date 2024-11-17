use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Learn more at https://github.com/nasa42/webterm", long_about = None)]
pub struct Args {
    /// Server ID, must be unique at least 16 characters long
    #[arg(long, env = "WT_SERVER_ID")]
    pub server_id: String,

    #[arg(long, env = "WT_SECRET_KEY")]
    pub secret_key: String,

    /// Relays to use, comma separated. If blank, default relays are used.
    #[arg(long, env = "WT_RELAYS")]
    pub relays: Option<String>,
}
