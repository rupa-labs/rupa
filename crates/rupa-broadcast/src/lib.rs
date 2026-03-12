//! # Rupa Broadcast 📡
//!
//! Global reactive event bus and channel-based communication for the Rupa Framework. 
//! Defines the foundational traits (Ports) for decoupled system orchestration.

pub mod bus;
pub mod channel;

pub use bus::Bus;
pub use channel::Channel;
use std::sync::{Arc, RwLock};
use async_trait::async_trait;

/// The primary Port for global application communication.
///
/// Implementations must handle the dispatching of messages to multiple 
/// independent subscribers in a thread-safe manner.
#[async_trait]
pub trait Broadcaster<T: Send + Sync + Clone + 'static>: Send + Sync {
    /// Dispatches a message to all active subscribers.
    async fn dispatch(&self, message: T);
    
    /// Subscribes to the broadcast with a thread-safe callback.
    fn subscribe(&self, callback: Arc<dyn Fn(T) + Send + Sync + 'static>);
}

/// A mock implementation of the Broadcaster trait for TDD and headless testing.
pub struct MockBus<T: Send + Sync + Clone + 'static> {
    pub sent_messages: Arc<RwLock<Vec<T>>>,
}

impl<T: Send + Sync + Clone + 'static> MockBus<T> {
    /// Creates a new, empty mock bus.
    pub fn new() -> Self {
        Self {
            sent_messages: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait]
impl<T: Send + Sync + Clone + 'static> Broadcaster<T> for MockBus<T> {
    async fn dispatch(&self, message: T) {
        self.sent_messages.write().unwrap().push(message);
    }

    fn subscribe(&self, _callback: Arc<dyn Fn(T) + Send + Sync + 'static>) {
        // Mock implementation does not trigger callbacks by default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_broadcast_flow() {
        let bus = MockBus::<String>::new();
        bus.dispatch("Hello Rupa".to_string()).await;
        
        let messages = bus.sent_messages.read().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "Hello Rupa");
    }
}
