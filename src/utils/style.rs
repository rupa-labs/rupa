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

    // --- Interactivity Utilities ---
    pub fn accent(mut self, color: impl Into<Color>) -> Self { self.interactivity.accent_color = Some(color.into()); self }
    pub fn appearance(mut self, val: Appearance) -> Self { self.interactivity.appearance = val; self }
    pub fn caret(mut self, color: impl Into<Color>) -> Self { self.interactivity.caret_color = Some(color.into()); self }
    pub fn color_scheme(mut self, val: ColorScheme) -> Self { self.interactivity.color_scheme = val; self }
    pub fn cursor(mut self, val: Cursor) -> Self { self.interactivity.cursor = val; self }
    pub fn pointer_events(mut self, val: PointerEvents) -> Self { self.interactivity.pointer_events = val; self }
    pub fn resize(mut self, val: Resize) -> Self { self.interactivity.resize = val; self }
    pub fn user_select(mut self, val: UserSelect) -> Self { self.interactivity.user_select = val; self }
    pub fn scroll_smooth(mut self) -> Self { self.interactivity.scroll_behavior = ScrollBehavior::Smooth; self }
    pub fn scroll_margin(mut self, val: impl IntoSpacing) -> Self { self.interactivity.scroll_margin = val.into_spacing(); self }
    pub fn scroll_padding(mut self, val: impl IntoSpacing) -> Self { self.interactivity.scroll_padding = val.into_spacing(); self }
    pub fn will_change(mut self, val: &str) -> Self { self.interactivity.will_change.push(val.to_string()); self }

    // --- SVG Utilities ---
    pub fn fill(mut self, color: impl Into<Color>) -> Self { self.svg.fill = Some(color.into()); self }
    pub fn stroke(mut self, color: impl Into<Color>) -> Self { self.svg.stroke = Some(color.into()); self }
    pub fn stroke_w(mut self, val: f32) -> Self { self.svg.stroke_width = Some(val); self }

    // --- Accessibility Utilities ---
    pub fn forced_color_adjust(mut self, val: ForcedColorAdjust) -> Self { self.accessibility.forced_color_adjust = val; self }

    // --- Common Box Model & Visuals ---
    pub fn bg(mut self, color: impl Into<Color>) -> Self { self.background.color = Some(color.into()); self }
    pub fn rounded(mut self, val: f32) -> Self { self.rounding = Rounding::all(val); self }
    pub fn border(mut self, width: f32, style: BorderStyle, color: impl Into<Color>) -> Self {
        self.border.width = width; self.border.style = style; self.border.color = Some(color.into()); self
    }
    pub fn w(mut self, val: f32) -> Self { self.sizing.width = Some(val); self }
    pub fn h(mut self, val: f32) -> Self { self.sizing.height = Some(val); self }
    pub fn p(mut self, val: impl IntoSpacing) -> Self { self.padding = val.into_spacing(); self }
    pub fn m(mut self, val: impl IntoSpacing) -> Self { self.margin = val.into_spacing(); self }
    pub fn color(mut self, color: impl Into<Color>) -> Self { self.typography.color = Some(color.into()); self }
    pub fn size(mut self, val: f32) -> Self { self.typography.size = Some(val); self }
    pub fn flex(mut self) -> Self { self.layout.display = Display::Flex; self }
    pub fn grid(mut self) -> Self { self.layout.display = Display::Grid; self }
}
