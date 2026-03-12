//! # Rupa Core 🧠
//!
//! The "Brain" of the Rupa Framework. This crate provides the **Composites** 
//! for virtual tree reconciliation, action dispatching, and component management.
//! It orchestrates Tier 1 Atoms into a functional, agnostic kernel.

pub mod component;
pub mod events;
pub mod renderer;
pub mod scene;
pub mod view;
pub mod reconciler;
pub mod action;

// Re-export Atomic Pieces
pub use rupa_base as support;
pub use rupa_base::{Vec2, Error, Id};

pub use rupa_signals as signals;
pub use rupa_signals::{Signal, Memo, Effect, Readable, CursorIcon};

pub use rupa_vnode as vnode;
pub use rupa_vnode::{VNode, VElement, VComponent, Style, Color, Attributes};

pub use component::Component;
pub use events::{InputEvent, KeyCode, PointerButton, ButtonState, UIEvent, Modifiers, EventListeners};
pub use renderer::{Renderer, TextMeasurer, RenderCore};
pub use scene::SceneNode;
pub use view::ViewCore;
pub use reconciler::{Patch, PatchSet, UpdateType};

use std::sync::{Arc, RwLock};

/// A thread-safe handle to a platform-specific Renderer.
pub type Port = Arc<dyn Renderer>;

/// A mock implementation of the Renderer for TDD and headless testing.
pub struct MockRenderer {
    pub patches_received: Arc<RwLock<Vec<Patch>>>,
}

impl MockRenderer {
    /// Creates a new, empty mock renderer.
    pub fn new() -> Self {
        Self {
            patches_received: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Renderer for MockRenderer {
    fn render_patch(&self, patches: PatchSet) {
        self.patches_received.write().unwrap().extend(patches);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_renderer_flow() {
        let renderer = MockRenderer::new();
        renderer.render_patch(vec![Patch::Delete { id: "node_1".into() }]);
        
        let received = renderer.patches_received.read().unwrap();
        assert_eq!(received.len(), 1);
        assert!(matches!(received[0], Patch::Delete { .. }));
    }
}
