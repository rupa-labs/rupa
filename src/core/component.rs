use taffy::prelude::*;
use crate::support::vector::Vec2;
use crate::renderer::{Renderer, TextMeasurer};
use crate::platform::dispatcher::UIEvent;
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::Cell;

/// The core trait for all UI elements in Rupaui.
pub trait Component: Send + Sync {
    fn id(&self) -> &str;
    fn children(&self) -> Vec<&dyn Component>;

    // Layout Infrastructure
    fn get_node(&self) -> Option<NodeId>;
    fn set_node(&self, node: NodeId);
    fn is_dirty(&self) -> bool;
    fn mark_dirty(&self);
    fn clear_dirty(&self);

    /// Compute the layout for this component and its children.
    /// Now includes a TextMeasurer to bridge with Layer 2.
    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId;

    /// Paint the component using the provided backend-agnostic renderer.
    fn paint(
        &self, 
        renderer: &mut dyn Renderer, 
        taffy: &TaffyTree<()>, 
        node: NodeId, 
        is_group_hovered: bool, 
        global_pos: Vec2
    );

    // Event hooks
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
