pub mod bus;
pub mod channel;

pub use bus::Bus;
pub use channel::Channel;
use std::sync::Arc;
use async_trait::async_trait;

/// The primary Port for global application communication.
#[async_trait]
pub trait Broadcaster<T: Send + Sync + Clone + 'static>: Send + Sync {
    /// Dispatches a message to all subscribers.
    async fn dispatch(&self, message: T);
    
    /// Subscribes to messages with a callback.
    fn subscribe(&self, callback: Arc<dyn Fn(T) + Send + Sync + 'static>);
}
