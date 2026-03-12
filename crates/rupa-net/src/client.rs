use crate::Client;
use rupa_base::Error;
use async_trait::async_trait;

/// A client that always fails or returns empty data.
/// Used as a placeholder before a real adapter is registered.
pub struct NullClient;

#[async_trait]
impl Client for NullClient {
    async fn get(&self, url: &str) -> Result<Vec<u8>, Error> {
        let _ = url;
        Err(Error::Platform("No network client registered".into()))
    }

    async fn post(&self, url: &str, _body: Vec<u8>) -> Result<Vec<u8>, Error> {
        let _ = url;
        Err(Error::Platform("No network client registered".into()))
    }
}
