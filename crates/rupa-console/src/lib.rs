pub mod ui;

pub use ui::Console;
pub use ui::text::Text;
pub use ui::layout::Layout;
pub use ui::progress::Progress;

/// Initialize the console for the current platform.
pub fn init() {
    // Platform-specific initialization if needed
}
