pub mod client;
pub mod resource;

pub use client::Client;
pub use resource::Resource;

/// Initiates a reactive network request.
pub fn fetch<T>(_url: &str) -> Resource<T> {
    todo!("Reactive fetch not yet implemented")
}
