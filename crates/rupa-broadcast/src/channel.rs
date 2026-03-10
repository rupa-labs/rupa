use std::sync::{Arc, RwLock};

/// A specialized stream for a specific event type.
pub struct Channel<T> {
    subscribers: RwLock<Vec<Arc<dyn Fn(&T) + Send + Sync>>>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            subscribers: RwLock::new(Vec::new()),
        }
    }

    /// Publishes a message to all active subscribers.
    pub fn publish(&self, message: T) {
        let subs = self.subscribers.read().unwrap();
        for sub in subs.iter() {
            (sub)(&message);
        }
    }

    /// Adds a new listener to the channel.
    pub fn subscribe<F>(&self, f: F) 
    where F: Fn(&T) + Send + Sync + 'static {
        let mut subs = self.subscribers.write().unwrap();
        subs.push(Arc::new(f));
    }
}
