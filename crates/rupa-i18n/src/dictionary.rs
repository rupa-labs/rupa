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

    /// Insert a translation into the dictionary.
    pub fn insert(&self, key: String, value: String) {
        let mut entries = self.entries.write().unwrap();
        entries.insert(key, value);
    }

    /// Retrieve a translation from the dictionary.
    pub fn get(&self, key: &str) -> Option<String> {
        let entries = self.entries.read().unwrap();
        entries.get(key).cloned()
    }

    /// Check if a translation exists.
    pub fn contains(&self, key: &str) -> bool {
        let entries = self.entries.read().unwrap();
        entries.contains_key(key)
    }

    /// Clear all translations.
    pub fn clear(&self) {
        let mut entries = self.entries.write().unwrap();
        entries.clear();
    }
}
