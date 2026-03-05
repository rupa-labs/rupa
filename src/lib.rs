pub mod app;
pub mod view;
pub mod elements;
pub mod utils;

pub use app::App;
pub use view::View;
pub use utils::Style;

/// The core trait for all Rupaui components.
pub trait Component {
    fn render(&self);
}
