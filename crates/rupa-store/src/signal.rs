use rupa_signals::{Signal, Readable};
use crate::store::Store;
use serde::Serialize;
use std::sync::Arc;

/// A reactive signal that automatically syncs its state to a persistent store.
pub struct PersistentSignal<T> {
    key: String,
    signal: Signal<T>,
    store: Arc<dyn Store>,
}

impl<T: Serialize + serde::de::DeserializeOwned + Clone + Send + Sync + 'static> PersistentSignal<T> {

    /// Creates a new persistent signal. 
    /// If the key exists in the store, it loads that value. 
    /// Otherwise, it uses the provided default and saves it.
    pub fn new(key: impl Into<String>, default: T, store: Arc<dyn Store>) -> Self {
        let key = key.into();
        
        // Attempt to load from store
        let initial_value = if let Ok(Some(bytes)) = store.read(&key) {
            serde_json::from_slice(&bytes).unwrap_or(default)
        } else {
            // Save default if not found
            let bytes = serde_json::to_vec(&default).unwrap_or_default();
            let _ = store.write(&key, &bytes);
            default
        };

        let signal = Signal::new(initial_value);

        // TODO: Register a global effect or internal listener to sync on change
        // For this MVP, we use a manual sync or we can hook into rupa-signals Effect.
        
        Self {
            key,
            signal,
            store,
        }
    }

    pub fn get(&self) -> T {
        self.signal.get()
    }

    pub fn set(&self, value: T) {
        self.signal.set(value.clone());
        let bytes = serde_json::to_vec(&value).unwrap_or_default();
        let _ = self.store.write(&self.key, &bytes);
    }
    }

    impl<T: Clone + Send + Sync + 'static + Serialize + serde::de::DeserializeOwned> Readable<T> for PersistentSignal<T> {
    fn get(&self) -> T { self.get() }
    }

