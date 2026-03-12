pub mod resource;
pub mod client;

pub use resource::Resource;
use std::sync::Arc;
use async_trait::async_trait;
use rupa_base::Error;

/// The primary Port for HTTP networking.
#[async_trait]
pub trait Client: Send + Sync {
    async fn get(&self, url: &str) -> Result<Vec<u8>, Error>;
    async fn post(&self, url: &str, body: Vec<u8>) -> Result<Vec<u8>, Error>;
}

pub type Port = Arc<dyn Client>;
