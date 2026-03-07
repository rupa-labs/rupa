use taffy::prelude::*;
use crate::support::vector::Vec2;
use crate::renderer::{Renderer, TextMeasurer};
use crate::platform::dispatcher::UIEvent;
use crate::scene::SceneNode;

/// The core trait for all UI elements in Rupaui.
pub trait Component: Send + Sync {
    fn id(&self) -> &str;
    fn children(&self) -> Vec<&dyn Component>;

    // Layout Infrastructure
    fn get_node(&self) -> Option<SceneNode>;
    fn set_node(&self, node: SceneNode);
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
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_release(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _delta: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _delta: Vec2) {}
    fn on_key(&self, _event: &mut UIEvent, _key: crate::platform::events::KeyCode) {}
    fn on_text(&self, _event: &mut UIEvent, _text: &str) { }
    fn on_resize(&self, _event: &mut UIEvent, _size: Vec2) {}
    
    // Lifecycle hooks
    fn on_mouse_enter(&self) {}
    fn on_mouse_leave(&self) {}

    /// Provides accessibility metadata for screen readers.
    fn accessibility(&self) -> crate::platform::AccessibilityNode {
        crate::platform::AccessibilityNode::default()
    }

    fn render(&self) {
        log::trace!("Component::render() called for {}", self.id());
    }
}
