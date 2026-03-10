use rupa_base::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Loader: Send + Sync {
    async fn load(&self, path: &str) -> Result<Vec<u8>, Error>;
}
