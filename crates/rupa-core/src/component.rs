use rupa_vnode::VNode;
use std::sync::Arc;
use crate::view::ViewCore;

/// The core trait for all UI elements in Rupa.
/// Focuses on logical state and producing a Virtual Node representation.
pub trait Component: Send + Sync {
    /// Returns a unique identifier for this component instance.
    fn id(&self) -> &str;

    /// Returns the reactive core of the component.
    fn view_core(&self) -> Arc<ViewCore>;

    /// Produces a Virtual Node representation of this component.
    /// This is the "Universal Language" used for SSR, DOM, and GPU rendering.
    fn render(&self) -> VNode;

    /// Optional: Returns logical children components (not VNodes).
    fn children(&self) -> Vec<&dyn Component> { Vec::new() }

    /// Optional: Allows downcasting trait objects to concrete types.
    fn as_any(&self) -> Option<&dyn std::any::Any> { None }

    /// If true, this component acts as a focus trap and blocks input to layers below.
    fn is_modal(&self) -> bool { false }

    // --- Lifecycle Helpers ---

    fn get_prev_vnode(&self) -> VNode {
        self.view_core().get_prev_vnode()
    }

    fn set_prev_vnode(&self, node: VNode) {
        self.view_core().set_prev_vnode(node);
    }

    fn is_dirty(&self) -> bool {
        self.view_core().is_dirty()
    }

    fn mark_dirty(&self) {
        self.view_core().mark_dirty();
    }

    fn clear_dirty(&self) {
        self.view_core().clear_dirty();
    }
}
