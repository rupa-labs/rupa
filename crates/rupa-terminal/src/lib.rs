//! # Rupa Terminal 📟
//!
//! The Terminal Showroom for the Rupa Framework. This crate provides the 
//! **Adapters & Infrastructure** (Tier 3) to run Rupa applications 
//! directly in the CLI using ANSI escape sequences and Crossterm.

pub mod runner;

pub use runner::TerminalRunner;
pub use rupa_console::Console;
pub use rupa_tui::TerminalRenderer;

/// The standard prelude for building Rupa Terminal applications.
pub mod prelude {
    pub use rupa_core::{Component, Signal, Readable};
    pub use rupa_engine::App;
    pub use rupa_console::Console;
    pub use rupa_ui::elements::{VStack, HStack, Text, Button};
    pub use super::TerminalRunner;
}
