//! # Rupa Core 🧠
//!
//! The "Brain" of the Rupa Framework. This crate provides the **Composites**
//! for virtual tree reconciliation, action dispatching, and component management.
//! It orchestrates Tier 1 Atoms into a functional, agnostic kernel.

pub mod action;
pub mod component;
pub mod events;
pub mod reconciler;
pub mod renderer;
pub mod scene;

// Re-export Atomic Pieces
pub use rupa_base as support;
pub use rupa_base::{Error, Id, Vec2};

pub use rupa_signals as signals;
pub use rupa_signals::{CursorIcon, Effect, Memo, Readable, Signal};

pub use rupa_vnode as vnode;
pub use rupa_vnode::{Attributes, Color, Style, VComponent, VElement, VNode};

pub use component::Component;
pub use events::{
    ButtonState, EventListeners, InputEvent, KeyCode, Modifiers, PointerButton, UIEvent,
};
pub use reconciler::{Patch, PatchSet, UpdateType};
pub use renderer::{MockRenderer, RenderCore, Renderer, TextMeasurer};
pub use scene::SceneNode;

use std::sync::Arc;

/// A thread-safe handle to a platform-specific Renderer.
pub type Port = Arc<dyn Renderer>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_renderer_flow() {
        let mut renderer = MockRenderer::new();
        renderer.render_patch(Patch::Delete {
            id: "node_1".into(),
        });

        let received = renderer.patches_received.read().unwrap();
        assert_eq!(received.len(), 1);
        assert!(matches!(received[0], Patch::Delete { .. }));
    }
}
