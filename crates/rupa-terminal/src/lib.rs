pub mod runner;

pub use runner::TerminalRunner;
pub use rupa_console::Console;
pub use rupa_tui::TerminalRenderer;

pub mod prelude {
    pub use rupa_core::{Component, Signal, Readable};
    pub use rupa_engine::App;
    pub use rupa_console::Console;
    pub use rupa_ui::elements::{VStack, HStack, Text, Button};
    pub use super::TerminalRunner;
}
