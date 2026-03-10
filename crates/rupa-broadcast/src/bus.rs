use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::any::Any;
use once_cell::sync::Lazy;
use crate::channel::Channel;

/// A thread-safe, multi-channel event bus.
pub struct BroadcastBus {
    channels: RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl BroadcastBus {
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
        }
    }

    /// Gets or creates a named channel for a specific event type.
    pub fn channel<T: Send + Sync + 'static>(&self, name: &str) -> Arc<Channel<T>> {
        let mut map = self.channels.write().unwrap();
        let entry = map.entry(name.to_string()).or_insert_with(|| {
            Arc::new(Channel::<T>::new())
        });
        
        entry.clone().downcast::<Channel<T>>().expect("Channel type mismatch")
    }

    /// Global singleton instance for the broadcast bus.
    pub fn global() -> &'static Self {
        static INSTANCE: Lazy<BroadcastBus> = Lazy::new(BroadcastBus::new);
        &INSTANCE
    }
}
