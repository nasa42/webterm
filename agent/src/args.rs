use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Learn more at https://github.com/nasa42/webterm", long_about = None)]
pub struct Args {
    /// Device Name, must be at least 16 characters long
    #[arg(long, env = "WT_DEVICE_NAME")]
    pub device_name: String,

    #[arg(long, env = "WT_SECRET_KEY")]
    pub secret_key: String,

    /// Relays to use, comma separated. If blank, default relays are used.
    /// If present, none of the default relays would be used.
    /// E.g., --relays relay1.example.com,relay2.example.com
    #[arg(long, env = "WT_RELAYS")]
    pub relays: Option<String>,

    #[arg(long, env = "WT_DEVICE_SUBNAME")]
    pub device_subname: Option<String>,

    #[arg(long, env = "WT_DAEMON", short = 'd')]
    pub daemon: bool,
}
