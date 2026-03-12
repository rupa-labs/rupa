use rupa_signals::{Signal, Readable, Effect};
use crate::store::Store;
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;

/// A reactive signal that automatically syncs its state to a persistent store.
pub struct PersistentSignal<T> {
    _key: String,
    signal: Signal<T>,
    _store: Arc<dyn Store>,
    _effect: Effect,
}

impl<T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static> PersistentSignal<T> {

    /// Creates a new persistent signal linked to a specific key in the store.
    /// If the key exists, it loads that value; otherwise, it saves the default.
    pub fn new(key: impl Into<String>, default: T, store: Arc<dyn Store>) -> Self {
        let key = key.into();
        
        // 1. Initial Load from Store
        let initial_value = if let Ok(Some(bytes)) = store.read(&key) {
            serde_json::from_slice(&bytes).unwrap_or(default)
        } else {
            // Initial save if not found
            let bytes = serde_json::to_vec(&default).unwrap_or_default();
            let _ = store.write(&key, &bytes);
            default
        };

        let signal = Signal::new(initial_value);
        
        // 2. Setup Automatic Syncing via Effect
        let s_clone = signal.clone();
        let store_clone = store.clone();
        let key_clone = key.clone();
        
        let effect = Effect::new(move || {
            // Track the signal
            let value = s_clone.get();
            
            // Serialize and save to store
            if let Ok(bytes) = serde_json::to_vec(&value) {
                let _ = store_clone.write(&key_clone, &bytes);
            }
        });

        Self {
            _key: key,
            signal,
            _store: store,
            _effect: effect,
        }
    }

    /// Returns a reference to the underlying reactive signal.
    pub fn signal(&self) -> &Signal<T> {
        &self.signal
    }

    pub fn get(&self) -> T {
        self.signal.get()
    }

    pub fn set(&self, value: T) {
        self.signal.set(value);
    }

    pub fn update<F>(&self, f: F)
    where F: FnOnce(&mut T) {
        self.signal.update(f);
    }
}

impl<T: Clone + Send + Sync + 'static + Serialize + DeserializeOwned> Readable<T> for PersistentSignal<T> {
    fn get(&self) -> T { self.get() }
}
