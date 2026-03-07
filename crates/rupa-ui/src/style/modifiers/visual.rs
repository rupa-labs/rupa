use rupa_core::style_data::Style;
use rupa_core::color::Color;
use rupa_core::border::Rounding;
use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn bg(color: Color) -> impl StyleModifier { move |s: &mut Style| s.background.color = Some(color.clone()) }
pub fn text_color(color: Color) -> impl StyleModifier { move |s: &mut Style| s.typography.color = Some(color.clone()) }
pub fn rounded(val: f32) -> impl StyleModifier { move |s: &mut Style| s.rounding = Rounding::all(val) }
pub fn rounded_full() -> impl StyleModifier { move |s: &mut Style| s.rounding = Rounding::all(9999.0) }

// --- Chaining API ---

pub trait ChainedVisual: Stylable {
    fn bg(self, color: Color) -> Self { self.style(bg(color)) }
    fn text_color(self, color: Color) -> Self { self.style(text_color(color)) }
    fn rounded(self, val: f32) -> Self { self.style(rounded(val)) }
    fn rounded_full(self) -> Self { self.style(rounded_full()) }
}

impl<T: Stylable> ChainedVisual for T {}
