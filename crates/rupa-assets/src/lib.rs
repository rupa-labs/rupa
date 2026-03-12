//! # Rupa Assets 📦
//!
//! Resource Loading and Caching for the Rupa Framework. 
//! Provides a reactive, asynchronous API for managing application assets 
//! across different platforms.

pub mod manager;
pub mod loader;
pub mod cache;

pub use manager::{Manager, Stats};
pub use loader::Loader;
pub use cache::Cache;

use rupa_signals::Signal;
use rupa_context::use_context;
use std::sync::{Arc, RwLock};
use async_trait::async_trait;
use rupa_base::Error;

/// Reactive hook to load an asset asynchronously.
/// 
/// Returns a `Signal<Option<Vec<u8>>>` that will automatically contain 
/// the asset data once the background loading is complete.
///
/// # Examples
///
/// ```
/// use rupa_assets::use_asset;
/// let logo = use_asset("assets/logo.png");
/// ```
pub fn use_asset(path: impl Into<String>) -> Signal<Option<Vec<u8>>> {
    let path = path.into();
    let asset_signal = Signal::new(None);
    
    if let Some(manager) = use_context::<Manager>() {
        let sig = asset_signal.clone();
        let p = path.clone();
        let manager = manager.clone();
        
        tokio::spawn(async move {
            if let Ok(data) = manager.load_raw(&p).await {
                sig.set(Some(data));
            }
        });
    }
    
    asset_signal
}

/// A mock implementation of the Asset Loader for TDD and headless testing.
pub struct MockLoader {
    pub response: Arc<RwLock<Vec<u8>>>,
}

#[async_trait]
impl Loader for MockLoader {
    fn can_handle(&self, _path: &str) -> bool { true }
    async fn load(&self, _path: &str) -> Result<Vec<u8>, Error> {
        Ok(self.response.read().unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_asset_loading() {
        let loader = Arc::new(MockLoader { response: Arc::new(RwLock::new(b"rupa-logo".to_vec())) });
        let res = loader.load("logo.png").await.unwrap();
        assert_eq!(res, b"rupa-logo");
    }
}
