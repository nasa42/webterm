use crate::args::Args;
use crate::models::relay::Relay;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use tracing::error;
use webterm_core::random::random_in_range;

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
}

impl Config {
    pub fn new(args: Args) -> Self {
        let available_relays = Self::init_available_relays(&args);
        Self {
            args,
            available_relays,
        }
    }

    pub fn server_id(&self) -> &String {
        &self.args.server_id
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
                    .map(|s| match Relay::new(&s) {
                        Ok(relay) => Some(Arc::new(relay)),
                        Err(e) => {
                            error!("Failed to create relay for {}: {}", s, e);
                            None
                        }
                    })
                    .filter_map(|relay| relay)
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
