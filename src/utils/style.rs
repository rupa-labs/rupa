use super::spacing::{Spacing, IntoSpacing};
use super::border::{Border, Rounding, Outline, BorderStyle};
use super::background::{Background, BackgroundAttachment, BackgroundClip, BackgroundOrigin, BackgroundRepeat, BackgroundSize};
use super::color::Color;
pub use super::layout::*;
pub use super::flex::*;
pub use super::grid::*;
pub use super::sizing::*;
pub use super::typography::*;
pub use super::effects::*;
pub use super::filters::*;
pub use super::table::*;
pub use super::animation::*;
pub use super::transform::*;
pub use super::interactivity::*;
pub use super::svg::*;
pub use super::accessibility::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style {
    pub layout: Layout,
    pub flex: Flex,
    pub grid: Grid,
    pub sizing: Sizing,
    pub typography: Typography,
    pub background: Background,
    pub border: Border,
    pub outline: Outline,
    pub rounding: Rounding,
    pub margin: Spacing,
    pub padding: Spacing,
    pub effects: Effects,
    pub filters: FilterStack,
    pub table: Table,
    pub motion: Motion,
    pub transform: Transform,
    pub interactivity: Interactivity,
    pub svg: Svg,
    pub accessibility: Accessibility,
}

impl Style {
    pub fn new() -> Self { Self::default() }

    // --- Helper Shorthands ---
    pub fn clearfix(mut self) -> Self { self.layout.clearfix = true; self }
    pub fn focus_ring(mut self) -> Self { self.effects.focus_ring = true; self }
    pub fn stretched_link(mut self) -> Self { self.interactivity.stretched_link = true; self }
    pub fn visually_hidden(mut self) -> Self { self.accessibility.visually_hidden = true; self }
    pub fn truncate(mut self) -> Self { self.typography.overflow = TextOverflow::Ellipsis; self.typography.white_space = WhiteSpace::NoWrap; self }

    // --- Sizing & Spacing ---
    pub fn w(mut self, val: f32) -> Self { self.sizing.width = Some(val); self }
    pub fn h(mut self, val: f32) -> Self { self.sizing.height = Some(val); self }
    pub fn m(mut self, val: impl IntoSpacing) -> Self { self.margin = val.into_spacing(); self }
    pub fn p(mut self, val: impl IntoSpacing) -> Self { self.padding = val.into_spacing(); self }
    
    // --- Visuals ---
    pub fn bg(mut self, color: impl Into<Color>) -> Self { self.background.color = Some(color.into()); self }
    pub fn color(mut self, color: impl Into<Color>) -> Self { self.typography.color = Some(color.into()); self }
    pub fn rounded(mut self, val: f32) -> Self { self.rounding = Rounding::all(val); self }
    
    // --- Layout ---
    pub fn flex(mut self) -> Self { self.layout.display = Display::Flex; self }
    pub fn aspect(mut self, val: f32) -> Self { self.layout.aspect_ratio = Some(val); self }
    pub fn z(mut self, val: i32) -> Self { self.layout.z_index = Some(val); self }
}
