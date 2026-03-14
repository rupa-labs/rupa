use rupa_base::Vec2;
pub use rupa_vnode::TextAlign;
pub use rupa_vnode::style::typography::TypographyStyle;
use crate::reconciler::Patch;

/// Shared internal state for all renderers.
pub struct RenderCore {
    pub logical_size: Vec2,
    pub camera_offset: Vec2,
    pub camera_zoom: f32,
    pub scale_factor: f32,
}

impl RenderCore {
    pub fn new(width: f32, height: f32, scale_factor: f32) -> Self {
        Self {
            logical_size: Vec2::new(width, height),
            camera_offset: Vec2::zero(),
            camera_zoom: 1.0,
            scale_factor,
        }
    }
}

/// A specialized trait for measuring text dimensions during the layout phase.
pub trait TextMeasurer: Send + Sync {
    fn measure(&self, text: &str, size: f32) -> Vec2;
}

/// The base contract for all Rupa rendering backends.
/// Provides a unified API for drawing primitives and executing structural patches.
pub trait Renderer: TextMeasurer {
    fn core(&self) -> &RenderCore;
    fn core_mut(&mut self) -> &mut RenderCore;

    /// Executes a structural UI instruction identified during reconciliation.
    fn render_patch(&mut self, patch: Patch);

    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4], radius: f32);
    fn draw_text(&mut self, text: &str, x: f32, y: f32, width: f32, style: &TypographyStyle);
    fn draw_outline(&mut self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4]);
    
    fn push_clip(&mut self, x: f32, y: f32, width: f32, height: f32);
    fn pop_clip(&mut self);
    
    fn present(&mut self);
}

use std::sync::{Arc, RwLock};

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
    fn draw_text(&mut self, _text: &str, _x: f32, _y: f32, _w: f32, _style: &TypographyStyle) {}
    fn draw_outline(&mut self, _x: f32, _y: f32, _w: f32, _h: f32, _color: [f32; 4]) {}
    fn push_clip(&mut self, _x: f32, _y: f32, _w: f32, _h: f32) {}
    fn pop_clip(&mut self) {}
    fn present(&mut self) {}
}
