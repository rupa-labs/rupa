//! # Rupa Net 🌐
//!
//! Async networking and resource handling for the Rupa Framework. 
//! Provides the foundational traits (Ports) for HTTP communication and 
//! reactive resource management.

pub mod resource;
pub mod client;

pub use resource::Resource;
use std::sync::Arc;
use async_trait::async_trait;
use rupa_base::Error;

/// The primary Port for HTTP networking.
///
/// This trait must be implemented by concrete Showroom Adapters (e.g., Reqwest, Fetch API).
#[async_trait]
pub trait Client: Send + Sync {
    /// Performs a GET request and returns the raw bytes.
    async fn get(&self, url: &str) -> Result<Vec<u8>, Error>;
    /// Performs a POST request with the given body and returns the raw bytes.
    async fn post(&self, url: &str, body: Vec<u8>) -> Result<Vec<u8>, Error>;
}

/// A thread-safe handle to a Networking Client.
pub type Port = Arc<dyn Client>;

/// A mock implementation of the Networking Client for TDD and headless testing.
pub struct MockClient {
    pub response: Vec<u8>,
}

#[async_trait]
impl Client for MockClient {
    async fn get(&self, _url: &str) -> Result<Vec<u8>, Error> {
        Ok(self.response.clone())
    }

    async fn post(&self, _url: &str, _body: Vec<u8>) -> Result<Vec<u8>, Error> {
        Ok(self.response.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_client_get() {
        let client = MockClient { response: b"hello rupa".to_vec() };
        let res = client.get("https://rupa.rs").await.unwrap();
        assert_eq!(res, b"hello rupa");
    }
}
