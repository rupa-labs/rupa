use std::collections::HashMap;
use std::sync::RwLock;

/// A thread-safe cache for binary resources.
pub struct Cache {
    data: RwLock<HashMap<String, Vec<u8>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self { data: RwLock::new(HashMap::new()) }
    }

    /// Insert a resource into the cache.
    pub fn insert(&self, key: String, value: Vec<u8>) {
        let mut data = self.data.write().unwrap();
        data.insert(key, value);
    }

    /// Retrieve a resource from the cache.
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let data = self.data.read().unwrap();
        data.get(key).cloned()
    }

    /// Check if a resource exists in the cache.
    pub fn contains(&self, key: &str) -> bool {
        let data = self.data.read().unwrap();
        data.contains_key(key)
    }

    /// Clear all resources from the cache.
    pub fn clear(&self) {
        let mut data = self.data.write().unwrap();
        data.clear();
    }
}
