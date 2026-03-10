use std::collections::HashMap;
use std::sync::RwLock;

/// A registry of translated strings.
pub struct Dictionary {
    entries: RwLock<HashMap<String, String>>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }
}
