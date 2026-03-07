use rupa_support::Vec2;
use crate::types::TextAlign;

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
/// Provides a unified API for drawing primitives like rectangles and text.
pub trait Renderer: TextMeasurer {
    fn core(&self) -> &RenderCore;
    fn core_mut(&mut self) -> &mut RenderCore;

    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4], radius: f32);
    fn draw_text(&mut self, text: &str, x: f32, y: f32, width: f32, size: f32, color: [f32; 4], align: TextAlign);
    fn draw_outline(&mut self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4]);
    
    fn push_clip(&mut self, x: f32, y: f32, width: f32, height: f32);
    fn pop_clip(&mut self);
    
    fn present(&mut self);
}
