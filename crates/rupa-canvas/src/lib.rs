//! # Rupa Canvas 🎨
//!
//! Low-level drawing ports and custom graphics for the Rupa Framework. 
//! Provides a platform-agnostic API for hardware-accelerated drawing.

pub mod api;
pub mod shaders;
pub mod pipeline;

pub use api::{Canvas, Command};
use std::sync::{Arc, RwLock};

/// The primary Port for low-level drawing operations.
///
/// This trait must be implemented by concrete Showroom Adapters (e.g., WGPU, Skia, HTML Canvas).
pub trait DrawTarget: Send + Sync {
    /// Executes a batch of drawing commands.
    fn execute(&self, commands: Vec<Command>);
}

/// A mock implementation of the DrawTarget for TDD and headless testing.
pub struct MockCanvas {
    pub commands: Arc<RwLock<Vec<Command>>>,
}

impl MockCanvas {
    /// Creates a new, empty mock canvas.
    pub fn new() -> Self {
        Self {
            commands: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl DrawTarget for MockCanvas {
    fn execute(&self, commands: Vec<Command>) {
        self.commands.write().unwrap().extend(commands);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_canvas_flow() {
        let canvas = MockCanvas::new();
        canvas.execute(vec![Command::Clear(rupa_base::Color::BLACK)]);
        
        let commands = canvas.commands.read().unwrap();
        assert_eq!(commands.len(), 1);
        assert!(matches!(commands[0], Command::Clear(_)));
    }
}
