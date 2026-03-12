use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Thread-safe asset cache.
pub struct Cache {
    data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, path: &str) -> Option<Vec<u8>> {
        let map = self.data.read().unwrap();
        map.get(path).cloned()
    }

    pub fn insert(&self, path: impl Into<String>, data: Vec<u8>) {
        let mut map = self.data.write().unwrap();
        map.insert(path.into(), data);
    }

    pub fn clear(&self) {
        let mut map = self.data.write().unwrap();
        map.clear();
    }

    pub fn remove(&self, path: &str) {
        let mut map = self.data.write().unwrap();
        map.remove(path);
    }
}
