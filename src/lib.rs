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
use crate::utils::Vec2;

pub trait Component {
    fn id(&self) -> &str;
    
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId;

    /// Paint phase now includes global_pos to handle relative-to-absolute coordinate mapping.
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2);

    fn on_click(&self);
    fn on_scroll(&self, delta: f32);
    fn on_drag(&self, delta: Vec2);

    fn render(&self) {
        log::trace!("Component::render() called for {}", self.id());
    }
}
