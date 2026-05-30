//! LeavePulse SDK — identity map.
//!
//! Guarantees one shared data cell per (resource, id). A resource holds an
//! `Arc<Mutex<Value>>`; the identity map hands back the same cell for the same
//! id, so a `refresh()` on one handle is visible to every handle. Per RFC
//! 0001 §5 it is not a TTL auto-refetcher — accessors read the cell, the
//! network is only touched on an explicit `refresh()` / `invalidate()`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_json::Value;

/// A shared, in-place-updatable data cell for one resource instance.
pub type DataCell = Arc<Mutex<Value>>;

/// One data cell per (type, id).
#[derive(Default)]
pub struct IdentityMap {
    by_type: Mutex<HashMap<String, HashMap<String, DataCell>>>,
}

impl IdentityMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the canonical cell for (type, id): update it in place if it
    /// exists, otherwise insert the given data.
    pub fn upsert(&self, type_: &str, id: &str, data: Value) -> DataCell {
        let mut guard = self.by_type.lock().expect("identity map poisoned");
        let bucket = guard.entry(type_.to_string()).or_default();
        if let Some(cell) = bucket.get(id) {
            *cell.lock().expect("cell poisoned") = data;
            return Arc::clone(cell);
        }
        let cell = Arc::new(Mutex::new(data));
        bucket.insert(id.to_string(), Arc::clone(&cell));
        cell
    }

    pub fn get(&self, type_: &str, id: &str) -> Option<DataCell> {
        let guard = self.by_type.lock().expect("identity map poisoned");
        guard.get(type_).and_then(|b| b.get(id)).map(Arc::clone)
    }

    pub fn invalidate(&self, type_: &str, id: Option<&str>) {
        let mut guard = self.by_type.lock().expect("identity map poisoned");
        match id {
            Some(id) => {
                if let Some(bucket) = guard.get_mut(type_) {
                    bucket.remove(id);
                }
            }
            None => {
                guard.remove(type_);
            }
        }
    }
}
