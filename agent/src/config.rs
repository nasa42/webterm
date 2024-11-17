use crate::args::Args;
use crate::models::relay::Relay;
use rand::Rng;

const DEFAULT_RELAYS: [&str; 1] = ["r1.relays.webterm.run"];

pub struct Config {
    args: Args,
    available_relays: Option<Vec<String>>,
    chosen_relay: Option<Relay>,
}

impl Config {
    pub fn new(args: Args) -> Self {
        Self {
            args,
            available_relays: None,
            chosen_relay: None,
        }
    }

    pub fn server_id(&self) -> &String {
        &self.args.server_id
    }

    pub fn secret_key(&self) -> &String {
        &self.args.secret_key
    }

    pub fn available_relays(&mut self) -> Vec<String> {
        if self.available_relays.is_none() {
            let result: Vec<String> = self
                .args
                .relays
                .as_ref()
                .map(|relays| {
                    relays
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect()
                })
                .unwrap_or_default();

            if result.is_empty() {
                self.available_relays =
                    Some(DEFAULT_RELAYS.iter().map(|s| s.to_string()).collect());
            } else {
                self.available_relays = Some(result);
            }
        };

        self.available_relays.clone().unwrap_or_default()
    }

    pub fn relay(&mut self) -> &Relay {
        if self.chosen_relay.is_none() {
            let chosen = self.random_relay();
            let relay = Relay::new(&self, &chosen).expect("Failed to create relay");
            self.chosen_relay = Some(relay);
        }

        self.chosen_relay.as_ref().unwrap()
    }

    fn random_relay(&mut self) -> String {
        let relays = self.available_relays();
        let index = rand::thread_rng().gen_range(0..relays.len());
        relays[index].clone()
    }
}
