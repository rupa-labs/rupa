use taffy::prelude::*;
use crate::vector::Vec2;
use crate::renderer::{Renderer, TextMeasurer};
use crate::events::{UIEvent, KeyCode};
use crate::scene::SceneNode;
use crate::a11y::AccessibilityNode;

/// The core trait for all UI elements in Rupa.
pub trait Component: Send + Sync {
    fn id(&self) -> &str;
    fn children(&self) -> Vec<&dyn Component>;

    /// Allows downcasting trait objects to concrete types.
    fn as_any(&self) -> &dyn std::any::Any;

    /// If true, this component acts as a focus trap and blocks input to layers below.
    fn is_modal(&self) -> bool { false }

    // Layout Infrastructure
    fn get_node(&self) -> Option<SceneNode>;
    fn set_node(&self, node: SceneNode);
    fn is_dirty(&self) -> bool;
    fn mark_dirty(&self);
    fn clear_dirty(&self);

    /// Compute the layout for this component and its children.
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
    fn on_key(&self, _event: &mut UIEvent, _key: KeyCode) {}
    fn on_text(&self, _event: &mut UIEvent, _text: &str) { }
    fn on_resize(&self, _event: &mut UIEvent, _size: Vec2) {}
    fn on_safe_area(&self, _event: &mut UIEvent, _top: f32, _right: f32, _bottom: f32, _left: f32) {}
    
    // Lifecycle hooks
    fn on_mouse_enter(&self) {}
    fn on_mouse_leave(&self) {}

    /// Provides accessibility metadata for screen readers.
    fn accessibility(&self) -> AccessibilityNode {
        AccessibilityNode::default()
    }

    fn render(&self) {
        log::trace!("Component::render() called for {}", self.id());
    }
}
