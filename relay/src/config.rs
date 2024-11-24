use std::sync::atomic::AtomicU64;
use std::sync::OnceLock;
use std::time::Duration;

// TODO: remove me once handshake is implemented
pub const TEST_SERVER_ID: &str = "test";

pub const WEBSOCKET_BUFFER_SIZE: usize = 1024 * 16;
pub const WEBSOCKET_MAX_BUFFER_SIZE: usize = 1024 * 64;
pub const MAX_AGENTS: usize = 50_000;
pub const MAX_FRONTENDS: usize = 50_000;
pub const HANDSHAKE_MAX_NONCES: usize = 50_000;
pub const HANDSHAKE_NONCE_EXPIRE_IN: Duration = Duration::from_secs(600);
pub const AGENT_EXPIRE_IN: Duration = Duration::from_secs(60);

static NEXT_CHANNEL_REQUEST_ID: AtomicU64 = AtomicU64::new(1);

pub struct Config {}

impl Config {
    fn singleton() -> &'static Self {
        static INSTANCE: OnceLock<Config> = OnceLock::new();
        INSTANCE.get_or_init(|| Self {})
    }

    pub fn next_channel_request_id() -> u64 {
        NEXT_CHANNEL_REQUEST_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}
