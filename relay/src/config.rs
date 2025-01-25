use std::time::Duration;

pub const WEBSOCKET_BUFFER_SIZE: usize = 1024 * 16;
pub const WEBSOCKET_MAX_BUFFER_SIZE: usize = 1024 * 64;
pub const MAX_AGENTS: usize = 50_000;
#[allow(dead_code)]
pub const MAX_FRONTENDS: usize = 50_000;
pub const HANDSHAKE_MAX_NONCES: usize = 50_000;
pub const HANDSHAKE_NONCE_EXPIRE_IN: Duration = Duration::from_secs(600);

pub const DEVICE_NAME_MIN_LENGTH: usize = 16;
pub const DEVICE_NAME_MAX_LENGTH: usize = 64;
pub const DEVICE_SUBNAME_MIN_LENGTH: usize = 1;
pub const DEVICE_SUBNAME_MAX_LENGTH: usize = 64;

// pub struct Config {}
//
// impl Config {
//     fn singleton() -> &'static Self {
//         static INSTANCE: OnceLock<Config> = OnceLock::new();
//         INSTANCE.get_or_init(|| Self {})
//     }
// }
