use std::time::Duration;

// TODO: remove me once handshake is implemented
pub const TEST_HANDSHAKE_NONCE: &str = "test";
pub const TEST_SERVER_ID: &str = "test";

pub const WEBSOCKET_BUFFER_SIZE: usize = 1024 * 16;
pub const WEBSOCKET_MAX_BUFFER_SIZE: usize = 1024 * 64;
pub const MAX_AGENTS: usize = 50_000;
pub const MAX_FRONTENDS: usize = 50_000;
pub const HANDSHAKE_MAX_NONCES: usize = 50_000;
pub const HANDSHAKE_NONCE_EXPIRE_IN: Duration = Duration::from_secs(600);
pub const AGENT_EXPIRE_IN: Duration = Duration::from_secs(60);
