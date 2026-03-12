pub mod manager;
pub mod loader;
pub mod cache;

pub use manager::{Manager, Stats};
pub use loader::Loader;
pub use cache::Cache;

use rupa_signals::Signal;
use rupa_context::use_context;
use std::sync::Arc;

/// Reactive hook to load an asset.
/// Returns a signal that will contain the bytes once loaded.
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
