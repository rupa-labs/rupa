use std::sync::{Arc, RwLock};
use async_trait::async_trait;
use crate::Broadcaster;

/// A specialized stream for a specific event type.
pub struct Channel<T: Send + Sync + Clone + 'static> {
    subscribers: RwLock<Vec<Arc<dyn Fn(T) + Send + Sync + 'static>>>,
}

impl<T: Send + Sync + Clone + 'static> Channel<T> {
    pub fn new() -> Self {
        Self {
            subscribers: RwLock::new(Vec::new()),
        }
    }
}

#[async_trait]
impl<T: Send + Sync + Clone + 'static> Broadcaster<T> for Channel<T> {
    /// Dispatches a message to all active subscribers.
    async fn dispatch(&self, message: T) {
        let subs = {
            let s = self.subscribers.read().unwrap();
            s.clone()
        };
        
        for sub in subs.iter() {
            (sub)(message.clone());
        }
    }

    /// Subscribes to messages with a callback.
    fn subscribe(&self, callback: Arc<dyn Fn(T) + Send + Sync + 'static>) {
        let mut subs = self.subscribers.write().unwrap();
        subs.push(callback);
    }
}
