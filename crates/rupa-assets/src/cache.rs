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
}
