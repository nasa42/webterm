use std::time::Duration;

// TODO: remove me once handshake is implemented
pub const TEST_SERVER_ID: &str = "test";

pub const WEBSOCKET_BUFFER_SIZE: usize = 1024 * 16;
pub const WEBSOCKET_MAX_BUFFER_SIZE: usize = 1024 * 64;
pub const MAX_AGENTS: usize = 50_000;
#[allow(dead_code)]
pub const MAX_FRONTENDS: usize = 50_000;
pub const HANDSHAKE_MAX_NONCES: usize = 50_000;
pub const HANDSHAKE_NONCE_EXPIRE_IN: Duration = Duration::from_secs(600);

// pub struct Config {}
//
// impl Config {
//     fn singleton() -> &'static Self {
//         static INSTANCE: OnceLock<Config> = OnceLock::new();
//         INSTANCE.get_or_init(|| Self {})
//     }
// }
