use crate::random::random_in_range;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::{Arc, PoisonError};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tokio::task::JoinHandle;
use tokio::time;
use tokio::time::{Duration, Instant};
use tracing::debug;

const MAX_CLEANUP_DURATION: Duration = Duration::from_millis(200);
const CLEANUP_EVERY: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub enum CacheError {
    ReadError,
    WriteError,
    AtCapacity,
    KeyNotFound,
}

impl std::error::Error for CacheError {}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::ReadError => write!(f, "Read error"),
            CacheError::WriteError => write!(f, "Write error"),
            CacheError::AtCapacity => write!(f, "Cache at capacity"),
            CacheError::KeyNotFound => write!(f, "Key not found"),
        }
    }
}

pub struct SimpleCache<K, V> {
    map: Arc<RwLock<HashMap<K, (V, Instant)>>>,
    max_size: usize, // since HashMap#len() returns usize
    cleanup_handle: JoinHandle<()>,
}

impl<K, V> SimpleCache<K, V>
where
    K: Hash + Eq + Send + Sync + Display + Debug + Clone + 'static,
    V: Send + Sync + Clone + 'static,
{
    pub fn new(max_size: usize) -> Self {
        let map = Arc::new(RwLock::new(HashMap::new()));
        let map_clone = map.clone();

        let cleanup_handle = tokio::spawn(async move {
            let mut interval = time::interval(CLEANUP_EVERY);
            loop {
                interval.tick().await;
                let _ = Self::remove_expired(&map_clone).await;
            }
        });

        SimpleCache {
            map,
            max_size,
            cleanup_handle,
        }
    }

    pub async fn len(&self) -> Result<usize, CacheError> {
        Ok(self.map.read().await.len())
    }

    pub async fn insert(&self, key: K, value: V, duration: Duration) -> Result<(), CacheError> {
        if self.len().await? >= self.max_size {
            return Err(CacheError::AtCapacity);
        }
        let expire_at = Instant::now() + duration;
        self.map.write().await.insert(key, (value, expire_at));
        Ok(())
    }

    pub async fn get(&self, key: &K) -> Result<V, CacheError> {
        debug!("starting simple_cache/get for key {:?}", key);
        let map = self.map.read().await;
        debug!("simple_cache/get map read lock acquired");
        let result = map.get(key).map(|(value, expires_at)| {
            debug!("simple_cache/get map.get() result: {:?}", expires_at);
            if &Instant::now() <= expires_at {
                Some(value)
            } else {
                None
            }
        });

        debug!("simple_cache/get map.get() result loop finished");

        if let Some(Some(value)) = result {
            debug!("simple_cache/get returning value");
            Ok(value.clone())
        } else {
            debug!("simple_cache/get returning key not found");
            Err(CacheError::KeyNotFound)
        }
    }

    pub async fn remove(&self, key: &K) -> Result<V, CacheError> {
        if let Some((value, _expires_at)) = self.map.write().await.remove(key) {
            Ok(value)
        } else {
            Err(CacheError::KeyNotFound)
        }
    }

    pub async fn reset_expiration(&self, key: K, duration: Duration) -> Result<(), CacheError> {
        let existing = self.get(&key).await?;
        self.insert(key, existing, duration).await
    }

    pub async fn remove_expired(
        map: &Arc<RwLock<HashMap<K, (V, Instant)>>>,
    ) -> Result<(), CacheError> {
        let start_time = Instant::now();
        let mut keys_to_remove: Vec<K> = Vec::new();

        {
            for (key, (_value, expires_at)) in map.write().await.iter() {
                if start_time.elapsed() > MAX_CLEANUP_DURATION {
                    break;
                }
                if expires_at < &start_time {
                    keys_to_remove.push(key.clone());
                }
            }
        }

        {
            let mut write_guard = map.write().await;
            for key in keys_to_remove.clone() {
                write_guard.remove(&key);
            }
        }

        // shrink to fit at randomly every 100th iteration
        if random_in_range(0, 100) == 0 {
            map.write().await.shrink_to_fit();
        }

        Ok(())
    }
}

impl<K, V> Drop for SimpleCache<K, V> {
    fn drop(&mut self) {
        self.cleanup_handle.abort()
    }
}
