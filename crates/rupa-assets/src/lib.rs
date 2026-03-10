pub mod manager;
pub mod loader;
pub mod cache;

pub use manager::AssetManager;

/// Reactive hook to load an asset.
pub fn use_asset<T>(_path: &str) -> T {
    todo!("use_asset not yet implemented")
}
