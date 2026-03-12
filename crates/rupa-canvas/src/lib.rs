pub mod api;
pub mod shaders;
pub mod pipeline;

pub use api::{Canvas, Command};

/// The primary Port for low-level drawing operations.
pub trait DrawTarget: Send + Sync {
    fn execute(&self, commands: Vec<Command>);
}
