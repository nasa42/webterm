use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, PoisonError};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tokio::task::JoinHandle;
use tokio::time;
use tokio::time::{Duration, Instant};

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

impl<K, V> From<PoisonError<RwLockReadGuard<'_, HashMap<K, (V, Instant)>>>> for CacheError {
    fn from(_: PoisonError<RwLockReadGuard<'_, HashMap<K, (V, Instant)>>>) -> Self {
        CacheError::ReadError
    }
}

impl<K, V> From<PoisonError<RwLockWriteGuard<'_, HashMap<K, (V, Instant)>>>> for CacheError {
    fn from(_: PoisonError<RwLockWriteGuard<'_, HashMap<K, (V, Instant)>>>) -> Self {
        CacheError::WriteError
    }
}

pub struct SimpleCache<K, V> {
    map: Arc<RwLock<HashMap<K, (V, Instant)>>>,
    max_size: usize, // since HashMap#len() returns usize
    cleanup_handle: JoinHandle<()>,
}

impl<K, V> SimpleCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
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
        let map = self.map.read().await;
        let result = map.get(key).map(|(value, expires_at)| {
            if &Instant::now() <= expires_at {
                Some(value)
            } else {
                None
            }
        });

        if let Some(Some(value)) = result {
            Ok(value.clone())
        } else {
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
        if let Some((value, _)) = self.map.read().await.get(&key) {
            let expire_at = Instant::now() + duration;
            self.map
                .write()
                .await
                .insert(key, (value.clone(), expire_at));
        }

        Ok(())
    }

    pub async fn remove_expired(
        map: &Arc<RwLock<HashMap<K, (V, Instant)>>>,
    ) -> Result<(), CacheError> {
        let now = Instant::now();

        for (key, (_value, expires_at)) in map.read().await.iter() {
            if now.elapsed() > MAX_CLEANUP_DURATION {
                break;
            }
            if expires_at < &now {
                map.write().await.remove(key);
            }
        }

        // shrink to fit at randomly every 100th iteration
        if rand::thread_rng().gen_range(0..100) == 0 {
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
