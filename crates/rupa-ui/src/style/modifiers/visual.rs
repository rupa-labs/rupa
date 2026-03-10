use rupa_vnode::{Style, Color, Rounding, FontWeight};
use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn bg(color: Color) -> impl StyleModifier {
    move |s: &mut Style| s.background.color = Some(color.clone())
}

pub fn bg_primary() -> impl StyleModifier {
    move |s: &mut Style| s.background.color = Some(Color::Semantic("primary".into(), None))
}

pub fn bg_surface() -> impl StyleModifier {
    move |s: &mut Style| s.background.color = Some(Color::Semantic("surface".into(), None))
}

pub fn rounded(val: f32) -> impl StyleModifier {
    move |s: &mut Style| s.rounding = Rounding::all(val)
}

pub fn text_color(color: Color) -> impl StyleModifier {
    move |s: &mut Style| s.typography.color = Some(color.clone())
}

pub fn font_bold() -> impl StyleModifier {
    move |s: &mut Style| s.typography.weight = FontWeight::Bold
}

pub fn font_size(val: f32) -> impl StyleModifier {
    move |s: &mut Style| s.typography.size = val
}

// --- Chaining API ---

pub trait ChainedVisual: Stylable {
    fn bg(self, color: Color) -> Self { self.style(bg(color)) }
    fn bg_primary(self) -> Self { self.style(bg_primary()) }
    fn bg_surface(self) -> Self { self.style(bg_surface()) }
    fn rounded(self, val: f32) -> Self { self.style(rounded(val)) }
    fn text_color(self, color: Color) -> Self { self.style(text_color(color)) }
    fn font_bold(self) -> Self { self.style(font_bold()) }
    fn font_size(self, val: f32) -> Self { self.style(font_size(val)) }
}

impl<T: Stylable> ChainedVisual for T {}
