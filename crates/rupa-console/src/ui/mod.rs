pub mod text;
pub mod layout;
pub mod progress;

pub use text::ConsoleText;
pub use layout::ConsoleLayout;
pub use progress::ConsoleProgress;

/// The standardized console facade for all Rupa terminal interactions.
pub struct Console;

impl Console {
    pub fn info(msg: impl AsRef<str>) { ConsoleText::info(msg); }
    pub fn success(msg: impl AsRef<str>) { ConsoleText::success(msg); }
    pub fn warning(msg: impl AsRef<str>) { ConsoleText::warning(msg); }
    pub fn error(msg: impl AsRef<str>) { ConsoleText::error(msg); }
    pub fn text(msg: impl AsRef<str>) { ConsoleText::plain(msg); }

    pub fn draw_line() { ConsoleLayout::draw_line(); }
    pub fn draw_box(title: &str, lines: Vec<String>) { ConsoleLayout::draw_box(title, lines); }

    pub fn draw_progress(label: &str, current: f32, max: f32) { ConsoleProgress::draw_bar(label, current, max); }
}
