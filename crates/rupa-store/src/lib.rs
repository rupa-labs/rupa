//! # Rupa Store 📦
//!
//! Persistent state and storage backends for the Rupa Framework. 
//! Integrates seamlessly with `rupa-signals` to provide reactive persistence.

pub mod store;
pub mod signal;
pub mod encryption;
pub mod backends;

pub use store::Store;
pub use signal::PersistentSignal;
pub use backends::fs::FileSystemStore;

#[cfg(target_arch = "wasm32")]
pub use backends::web::WebStorageStore;

use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use rupa_base::Error;

/// A thread-safe handle to a Storage Service.
pub type Port = Arc<dyn Store>;

/// A mock in-memory implementation of the Store trait for TDD and headless testing.
pub struct MockStore {
    data: RwLock<HashMap<String, Vec<u8>>>,
}

impl MockStore {
    /// Creates a new, empty in-memory store.
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }
}

impl Store for MockStore {
    fn read(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        Ok(self.data.read().unwrap().get(key).cloned())
    }

    fn write(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        self.data.write().unwrap().insert(key.to_string(), value.to_vec());
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        self.data.write().unwrap().remove(key);
        Ok(())
    }

    fn clear(&self) -> Result<(), Error> {
        self.data.write().unwrap().clear();
        Ok(())
    }

    fn keys(&self) -> Result<Vec<String>, Error> {
        Ok(self.data.read().unwrap().keys().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Deserialize};

    #[test]
    fn test_persistent_signal_with_mock_store() {
        let store = Arc::new(MockStore::new());
        
        #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
        struct Config { theme: String }
        
        let s = PersistentSignal::new("config", Config { theme: "dark".into() }, store.clone());
        assert_eq!(s.get().theme, "dark");
        
        s.set(Config { theme: "light".into() });
        
        // Create a new signal pointing to the same key
        let s2 = PersistentSignal::new("config", Config::default(), store);
        assert_eq!(s2.get().theme, "light");
    }
}
