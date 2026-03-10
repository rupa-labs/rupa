pub mod store;
pub mod signal;
pub mod backends;
pub mod encryption;

pub use store::Store;
pub use signal::PersistentSignal;

/// Automatically initializes the best storage backend for the current platform.
pub fn auto_init() -> Box<dyn Store> {
    // Placeholder for platform-specific logic
    todo!("Automatic storage initialization not yet implemented")
}
