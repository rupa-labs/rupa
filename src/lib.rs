pub mod app;
pub mod section;
pub mod elements;
pub mod utils;
pub mod window;
pub mod container;
pub mod plugin;
pub mod renderer;
pub mod prelude;

pub use app::App;
pub use section::Section;
pub use utils::Style;
pub use window::Window;
pub use plugin::Plugin;
pub use renderer::Renderer;

use taffy::prelude::*;

/// The core trait for all Rupaui components.
/// Ensures Separation of Concerns between ID management, layout, and rendering.
pub trait Component {
    /// Returns the unique identity of the component.
    fn id(&self) -> &str;
    
    /// The layout phase where the component defines its size and position in the Taffy tree.
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId;

    /// The primary rendering phase where the component draws its primitives using the Renderer.
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>);

    /// Handles click events.
    fn on_click(&self);

    /// Handles scroll/wheel events.
    fn on_scroll(&self, delta: f32);

    /// Handles drag/mouse move events with delta.
    fn on_drag(&self, delta: crate::utils::Vec2);

    /// Utility to trigger a generic render (for backward compatibility or debugging).
    fn render(&self) {
        log::trace!("Component::render() called for {}", self.id());
    }
}
