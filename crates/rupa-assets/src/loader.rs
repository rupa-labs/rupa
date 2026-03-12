use rupa_base::Error;
use async_trait::async_trait;

/// A Port for loading raw assets from various sources.
#[async_trait]
pub trait Loader: Send + Sync {
    /// Returns true if this loader can handle the given path/protocol.
    fn can_handle(&self, path: &str) -> bool;

    /// Loads the raw bytes of the asset.
    async fn load(&self, path: &str) -> Result<Vec<u8>, Error>;
}
