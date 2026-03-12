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
    pub core: RenderCore,
}

impl MockRenderer {
    /// Creates a new, empty mock renderer.
    pub fn new() -> Self {
        Self {
            patches_received: Arc::new(RwLock::new(Vec::new())),
            core: RenderCore::new(800.0, 600.0, 1.0),
        }
    }
}

impl TextMeasurer for MockRenderer {
    fn measure(&self, text: &str, _size: f32) -> Vec2 {
        Vec2::new(text.len() as f32 * 10.0, 20.0)
    }
}

impl Renderer for MockRenderer {
    fn core(&self) -> &RenderCore { &self.core }
    fn core_mut(&mut self) -> &mut RenderCore { &mut self.core }

    fn render_patch(&mut self, patch: Patch) {
        self.patches_received.write().unwrap().push(patch);
    }

    fn draw_rect(&mut self, _x: f32, _y: f32, _w: f32, _h: f32, _color: [f32; 4], _radius: f32) {}
    fn draw_text(&mut self, _text: &str, _x: f32, _y: f32, _w: f32, _size: f32, _color: [f32; 4], _align: rupa_vnode::TextAlign) {}
    fn draw_outline(&mut self, _x: f32, _y: f32, _w: f32, _h: f32, _color: [f32; 4]) {}
    fn push_clip(&mut self, _x: f32, _y: f32, _w: f32, _h: f32) {}
    fn pop_clip(&mut self) {}
    fn present(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_renderer_flow() {
        let mut renderer = MockRenderer::new();
        renderer.render_patch(Patch::Delete { id: "node_1".into() });
        
        let received = renderer.patches_received.read().unwrap();
        assert_eq!(received.len(), 1);
        assert!(matches!(received[0], Patch::Delete { .. }));
    }
}
