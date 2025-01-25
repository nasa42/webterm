use crate::args::Args;
use crate::models::relay::Relay;
use gethostname::gethostname;
use std::sync::Arc;
use std::time::Duration;
use tracing::error;
use webterm_core::random::{random_alphanumeric, random_in_range};

// should be same as in frontend/src/scripts/client/config.ts
const DEFAULT_RELAYS: [&str; 4] = [
    "r1.relays.webterm.run",
    "r2.relays.webterm.run",
    "r4.relays.webterm.run",
    "r5.relays.webterm.run",
];
pub const RELAY_RECONNECT_INTERVAL: Duration = Duration::from_secs(5);
pub const DEFAULT_PBKDF2_ITERATIONS: u32 = 100_000;

pub struct Config {
    args: Args,
    available_relays: Vec<Arc<Relay>>,
    original_device_subname: String,
}

impl Config {
    pub fn new(args: Args) -> Self {
        let available_relays = Self::init_available_relays(&args);
        let device_subname = infer_subname(&args);
        Self {
            args,
            available_relays,
            original_device_subname: device_subname.clone(),
        }
    }

    pub fn device_name(&self) -> &String {
        &self.args.device_name
    }

    pub fn original_device_subname(&self) -> &String {
        &self.original_device_subname
    }

    pub fn try_new_device_subname(&self) -> String {
        format!(
            "{}-{}",
            self.original_device_subname,
            random_alphanumeric(4)
        )
    }

    pub fn secret_key(&self) -> &String {
        &self.args.secret_key
    }

    fn init_available_relays(args: &Args) -> Vec<Arc<Relay>> {
        let mut result: Vec<Arc<Relay>> = args
            .relays
            .as_ref()
            .map(|relays| {
                relays
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .filter_map(|s| match Relay::new(&s) {
                        Ok(relay) => Some(Arc::new(relay)),
                        Err(e) => {
                            error!("Failed to create relay for {}: {}", s, e);
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        if result.is_empty() {
            result = DEFAULT_RELAYS
                .iter()
                .map(|s| Arc::new(Relay::new(s).unwrap()))
                .collect()
        }

        result
    }

    pub fn random_relay(&self) -> Arc<Relay> {
        let relays = &self.available_relays;
        let index = random_in_range(0, relays.len());
        relays[index].clone()
    }
}

fn infer_subname(args: &Args) -> String {
    args.device_subname
        .clone()
        .unwrap_or_else(|| get_hostname().unwrap_or_else(random_subname))
}

fn random_subname() -> String {
    format!("device-{}", random_alphanumeric(8))
}

fn get_hostname() -> Option<String> {
    let name = gethostname().to_string_lossy().to_string();
    if name.trim().is_empty() {
        None
    } else {
        Some(name)
    }
}
