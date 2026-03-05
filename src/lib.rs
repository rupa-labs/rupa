pub mod app;
pub mod section;
pub mod elements;
pub mod utils;
pub mod window;
pub mod container;

pub use app::App;
pub use section::Section;
pub use utils::Style;
pub use window::Window;

/// The core trait for all Rupaui components.
/// Ensures Separation of Concerns between ID management and rendering.
pub trait Component {
    /// Returns the unique identity of the component.
    fn id(&self) -> &str;
    
    /// The primary rendering phase (currently uses log-based render).
    /// Future refactor: split into .layout() and .paint() for WGPU/Taffy optimization.
    fn render(&self);
}
