//! # Rupa Console 💻
//!
//! Low-level terminal infrastructure and aesthetic CLI primitives for the 
//! Rupa Framework. Provides the foundational Atoms for terminal-based 
//! artisan experiences.

pub mod ui;

pub use ui::Console;
pub use ui::text::Text;
pub use ui::layout::Layout;
pub use ui::progress::Progress;

use std::sync::{Arc, RwLock};

/// Initialize the console for the current platform.
///
/// Sets up terminal state, character encoding, and event polling 
/// based on the execution environment.
pub fn init() {
    // Platform-specific initialization (e.g., enable raw mode)
}

/// A mock implementation of the Console for TDD and headless testing.
pub struct MockConsole {
    pub buffer: Arc<RwLock<Vec<String>>>,
}

impl MockConsole {
    /// Creates a new, empty mock console.
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Writes a line to the mock buffer.
    pub fn write_line(&self, line: impl Into<String>) {
        self.buffer.write().unwrap().push(line.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_console_output() {
        let console = MockConsole::new();
        console.write_line("Hello Artisan");
        
        let buffer = console.buffer.read().unwrap();
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer[0], "Hello Artisan");
    }
}
