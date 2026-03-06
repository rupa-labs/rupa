use crate::utils::style::Style;
use crate::utils::color::Color;
use crate::utils::spacing::IntoSpacing;
use crate::utils::scale::Scale;
pub use crate::utils::layout::{Display, Position, Overflow};
pub use crate::utils::flex::{FlexDirection, AlignItems, JustifyContent};

pub trait StyleModifier {
    fn apply(self, style: &mut Style);
}

impl StyleModifier for Style { fn apply(self, style: &mut Style) { *style = self; } }
impl<F> StyleModifier for F where F: FnOnce(&mut Style) { fn apply(self, style: &mut Style) { self(style) } }

// Tuple support
impl<A: StyleModifier, B: StyleModifier> StyleModifier for (A, B) {
    fn apply(self, style: &mut Style) { self.0.apply(style); self.1.apply(style); }
}
impl<A: StyleModifier, B: StyleModifier, C: StyleModifier> StyleModifier for (A, B, C) {
    fn apply(self, style: &mut Style) { self.0.apply(style); self.1.apply(style); self.2.apply(style); }
}
impl<A: StyleModifier, B: StyleModifier, C: StyleModifier, D: StyleModifier> StyleModifier for (A, B, C, D) {
    fn apply(self, style: &mut Style) { self.0.apply(style); self.1.apply(style); self.2.apply(style); self.3.apply(style); }
}
impl<A: StyleModifier, B: StyleModifier, C: StyleModifier, D: StyleModifier, E: StyleModifier> StyleModifier for (A, B, C, D, E) {
    fn apply(self, style: &mut Style) { self.0.apply(style); self.1.apply(style); self.2.apply(style); self.3.apply(style); self.4.apply(style); }
}

// --- Layout Utilities ---
pub fn block() -> impl StyleModifier { |s: &mut Style| s.layout.display = Display::Block }
pub fn flex() -> impl StyleModifier { |s: &mut Style| s.layout.display = Display::Flex }
pub fn grid() -> impl StyleModifier { |s: &mut Style| s.layout.display = Display::Grid }
pub fn hidden() -> impl StyleModifier { |s: &mut Style| s.layout.display = Display::None }

pub fn relative() -> impl StyleModifier { |s: &mut Style| s.layout.position = Position::Relative }
pub fn absolute() -> impl StyleModifier { |s: &mut Style| s.layout.position = Position::Absolute }

pub fn overflow_hidden() -> impl StyleModifier { |s: &mut Style| { s.layout.overflow_x = Overflow::Hidden; s.layout.overflow_y = Overflow::Hidden; } }
pub fn overflow_scroll() -> impl StyleModifier { |s: &mut Style| { s.layout.overflow_x = Overflow::Scroll; s.layout.overflow_y = Overflow::Scroll; } }

// --- Flex Utilities ---
pub fn row() -> impl StyleModifier { |s: &mut Style| s.flex.flex_direction = FlexDirection::Row }
pub fn col() -> impl StyleModifier { |s: &mut Style| s.flex.flex_direction = FlexDirection::Col }
pub fn items_center() -> impl StyleModifier { |s: &mut Style| s.flex.align_items = Some(AlignItems::Center) }
pub fn justify_center() -> impl StyleModifier { |s: &mut Style| s.flex.justify_content = Some(JustifyContent::Center) }
pub fn justify_between() -> impl StyleModifier { |s: &mut Style| s.flex.justify_content = Some(JustifyContent::SpaceBetween) }
pub fn flex_grow() -> impl StyleModifier { |s: &mut Style| s.flex.flex_grow = 1.0 }

pub fn gap(val: f32) -> impl StyleModifier { move |s: &mut Style| s.flex.gap = Some(val) }

// --- Sizing & Spacing ---
pub fn w(val: f32) -> impl StyleModifier { move |s: &mut Style| s.sizing.width = Some(val) }
pub fn h(val: f32) -> impl StyleModifier { move |s: &mut Style| s.sizing.height = Some(val) }
pub fn w_full() -> impl StyleModifier { |s: &mut Style| s.sizing.width = Some(100.0) }
pub fn h_full() -> impl StyleModifier { |s: &mut Style| s.sizing.height = Some(100.0) }

pub fn p(val: impl IntoSpacing) -> impl StyleModifier { move |s: &mut Style| s.padding = val.into_spacing() }
pub fn m(val: impl IntoSpacing) -> impl StyleModifier { move |s: &mut Style| s.margin = val.into_spacing() }

pub fn rounded(val: f32) -> impl StyleModifier { move |s: &mut Style| s.rounding = crate::utils::border::Rounding::all(val) }

// --- Background & Typography ---
pub fn bg(color: impl Into<Color>) -> impl StyleModifier {
    let color = color.into();
    move |s: &mut Style| s.background.color = Some(color.clone())
}

pub fn text_color(color: impl Into<Color>) -> impl StyleModifier {
    let color = color.into();
    move |s: &mut Style| s.typography.color = Some(color.clone())
}

pub fn text_sm() -> impl StyleModifier { |s: &mut Style| s.typography.size = Some(12.0) }
pub fn text_lg() -> impl StyleModifier { |s: &mut Style| s.typography.size = Some(20.0) }

// --- Scale Based ---
pub fn p_scale(s: Scale) -> impl StyleModifier { let val = s.value(16.0); move |style: &mut Style| style.padding = crate::utils::spacing::Spacing::all(val) }
pub fn gap_scale(s: Scale) -> impl StyleModifier { let val = s.value(16.0); move |style: &mut Style| style.flex.gap = Some(val) }
