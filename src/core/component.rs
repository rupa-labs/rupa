use taffy::prelude::*;
use crate::utils::vector::Vec2;
use crate::renderer::renderer::Renderer;
use crate::platform::events::UIEvent;
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::Cell;

/// The core trait for all Rupaui components.
pub trait Component {
    /// Unique identifier for the component instance.
    fn id(&self) -> &str;
    
    /// Returns a list of children components for traversal.
    fn children(&self) -> Vec<&dyn Component>;

    /// Gets the persistent Taffy NodeId associated with this component.
    fn get_node(&self) -> Option<NodeId>;
    
    /// Sets the persistent Taffy NodeId for this component.
    fn set_node(&self, node: NodeId);

    /// Checks if the component needs a layout update.
    fn is_dirty(&self) -> bool;
    
    /// Marks the component as needing a layout update.
    fn mark_dirty(&self);
    
    /// Clears the dirty flag.
    fn clear_dirty(&self);

    /// Layout phase: creates or updates Taffy nodes.
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId;

    /// Paint phase: renders the component using the provided renderer.
    fn paint(
        &self, 
        renderer: &mut Renderer, 
        taffy: &TaffyTree<()>, 
        node: NodeId, 
        is_group_hovered: bool, 
        render_pass: &mut wgpu::RenderPass<'_>, 
        global_pos: Vec2
    );

    // Interaction hooks with Event control
    fn on_click(&self, event: &mut UIEvent);
    fn on_scroll(&self, event: &mut UIEvent, delta: f32);
    fn on_drag(&self, event: &mut UIEvent, delta: Vec2);
    
    // Lifecycle hooks
    fn on_mouse_enter(&self) {}
    fn on_mouse_leave(&self) {}

    fn render(&self) {
        log::trace!("Component::render() called for {}", self.id());
    }
}
