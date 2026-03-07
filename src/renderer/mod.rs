use crate::support::vector::Vec2;
use crate::style::utilities::typography::TextAlign;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "tui")]
pub mod tui;

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
/// This prevents Layer 3 from needing full rendering knowledge.
pub trait TextMeasurer {
    fn measure(&self, text: &str, size: f32) -> Vec2;
}

/// The universal contract for all rendering backends.
pub trait Renderer: TextMeasurer {
    fn core(&self) -> &RenderCore;
    fn core_mut(&mut self) -> &mut RenderCore;

    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4], radius: f32);
    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, color: [f32; 4], align: TextAlign);
    
    fn push_clip(&mut self, x: f32, y: f32, width: f32, height: f32);
    fn pop_clip(&mut self);
    
    fn present(&mut self);
}

#[cfg(feature = "gui")]
pub use gui::renderer::Renderer as GuiRenderer;
#[cfg(feature = "tui")]
pub use tui::TuiRenderer;
