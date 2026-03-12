//! # Rupa Engine ⚙️
//!
//! The "Orchestrator" of the Rupa Framework. This crate provides the **Composites** 
//! for application lifecycle management, platform bridging, and plugin systems.
//! It acts as the kernel that connects Core logic to Artisan Showrooms.

pub mod platform;
pub mod scene;
pub mod plugin;
pub mod element_tree;
pub mod renderer;

pub use platform::app::App;
pub use plugin::{Plugin, PluginRegistry};
pub use element_tree::ElementTree;

use std::sync::{Arc, RwLock};

/// A mock implementation of the App for TDD and headless testing.
pub struct MockApp {
    pub is_running: Arc<RwLock<bool>>,
}

impl MockApp {
    /// Creates a new, empty mock application.
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    /// Simulates starting the application.
    pub fn run(&self) {
        *self.is_running.write().unwrap() = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_app_lifecycle() {
        let app = MockApp::new();
        assert_eq!(*app.is_running.read().unwrap(), false);
        app.run();
        assert_eq!(*app.is_running.read().unwrap(), true);
    }
}
