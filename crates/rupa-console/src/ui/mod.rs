pub mod text;
pub mod layout;
pub mod progress;

pub use text::Text;
pub use layout::Layout;
pub use progress::Progress;

/// The standardized console facade for all Rupa terminal interactions.
pub struct Console;

impl Console {
    pub fn info(msg: impl AsRef<str>) { Text::info(msg); }
    pub fn success(msg: impl AsRef<str>) { Text::success(msg); }
    pub fn warning(msg: impl AsRef<str>) { Text::warning(msg); }
    pub fn error(msg: impl AsRef<str>) { Text::error(msg); }
    pub fn text(msg: impl AsRef<str>) { Text::plain(msg); }

    pub fn draw_line() { Layout::draw_line(); }
    pub fn draw_box(title: &str, lines: Vec<String>) { Layout::draw_box(title, lines); }

    pub fn draw_progress(label: &str, current: f32, max: f32) { Progress::draw_bar(label, current, max); }
}
