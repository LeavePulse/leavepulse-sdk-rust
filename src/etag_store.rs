//! ETag cache store (RFC 7232 conditional requests).
//!
//! [`BearerTransport::conditional`] is the raw mechanism: it sends
//! `If-None-Match` and reports [`ConditionalOutcome`], but remembers nothing.
//! This adds the policy: an [`EtagStore`] keeps `{etag, body}` per key, and
//! [`fetch_cached`] ties it to the transport — serve the cached body on `304`,
//! store the new body on `200`. Two stores ship ([`MemoryEtagStore`],
//! [`FileEtagStore`]); implement [`EtagStore`] for anything else (SQLite, …).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::transport::{BearerTransport, Channel, ConditionalOutcome, Method, TransportError};

/// A cached conditional response: the validator and the body it belongs to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtagEntry {
    pub etag: String,
    pub body: Value,
}

/// Pluggable storage for ETag cache entries. Implementations must be
/// thread-safe (`Send + Sync`); a failing backend should degrade to a miss
/// (return `None` / silently skip) rather than error, so the cache never
/// breaks a request.
pub trait EtagStore: Send + Sync {
    fn get(&self, key: &str) -> Option<EtagEntry>;
    fn set(&self, key: &str, entry: EtagEntry);
    fn delete(&self, key: &str);
}

/// The default cache key for a request: channel + method + path.
pub fn default_cache_key(method: Method, path: &str, channel: Channel) -> String {
    let chan = match channel {
        Channel::Auth => "auth",
        Channel::Platform | Channel::PlatformPublic => "platform",
    };
    format!("{chan} {} {path}", method.as_str())
}

/// In-process cache. Lives for the process; ideal for tests and short-lived
/// runs. Persisted state needs [`FileEtagStore`] or a custom store.
#[derive(Default)]
pub struct MemoryEtagStore {
    map: Mutex<HashMap<String, EtagEntry>>,
}

impl MemoryEtagStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl EtagStore for MemoryEtagStore {
    fn get(&self, key: &str) -> Option<EtagEntry> {
        self.map.lock().ok()?.get(key).cloned()
    }

    fn set(&self, key: &str, entry: EtagEntry) {
        if let Ok(mut map) = self.map.lock() {
            map.insert(key.to_string(), entry);
        }
    }

    fn delete(&self, key: &str) {
        if let Ok(mut map) = self.map.lock() {
            map.remove(key);
        }
    }
}

/// File-persistent cache: one JSON file holding all entries, under `dir`.
/// Entries survive restarts. Any IO/parse failure degrades to a miss so a
/// missing dir or corrupt file never breaks a request. Suitable for desktop
/// apps (e.g. the launcher) that want an offline-capable cache without pulling
/// in a database.
pub struct FileEtagStore {
    path: PathBuf,
    cache: Mutex<HashMap<String, EtagEntry>>,
}

impl FileEtagStore {
    /// Open (or lazily create) a cache file `etag-cache.json` in `dir`.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        let path = dir.into().join("etag-cache.json");
        let cache = std::fs::read(&path)
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default();
        Self {
            path,
            cache: Mutex::new(cache),
        }
    }

    fn flush(&self, map: &HashMap<String, EtagEntry>) {
        if let Ok(bytes) = serde_json::to_vec(map) {
            if let Some(parent) = self.path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&self.path, bytes);
        }
    }
}

impl EtagStore for FileEtagStore {
    fn get(&self, key: &str) -> Option<EtagEntry> {
        self.cache.lock().ok()?.get(key).cloned()
    }

    fn set(&self, key: &str, entry: EtagEntry) {
        if let Ok(mut map) = self.cache.lock() {
            map.insert(key.to_string(), entry);
            self.flush(&map);
        }
    }

    fn delete(&self, key: &str) {
        if let Ok(mut map) = self.cache.lock() {
            map.remove(key);
            self.flush(&map);
        }
    }
}

/// Fetch a resource through the ETag cache: send the stored validator, return
/// the cached body on `304`, store and return the fresh body on `200`, and
/// return `None` on `404` (evicting any stale entry).
pub async fn fetch_cached(
    transport: &BearerTransport,
    store: &dyn EtagStore,
    method: Method,
    path: &str,
    channel: Channel,
) -> Result<Option<Value>, TransportError> {
    let key = default_cache_key(method, path, channel);
    let cached = store.get(&key);
    let prior_etag = cached.as_ref().map(|e| e.etag.as_str());
    match transport
        .conditional(method, path, channel, prior_etag)
        .await?
    {
        ConditionalOutcome::NotModified { .. } => Ok(cached.map(|e| e.body)),
        ConditionalOutcome::Modified { data, etag } => {
            if let Some(etag) = etag {
                store.set(
                    &key,
                    EtagEntry {
                        etag,
                        body: data.clone(),
                    },
                );
            }
            Ok(Some(data))
        }
        ConditionalOutcome::NotFound => {
            store.delete(&key);
            Ok(None)
        }
    }
}
