use crate::utils::style::Style;
use crate::utils::color::Color;
use crate::utils::spacing::IntoSpacing;

/// A functional modifier that can be applied to a Style object.
pub trait StyleModifier {
    fn apply(self, style: &mut Style);
}

// Support for a full Style object as a modifier
impl StyleModifier for Style {
    fn apply(self, style: &mut Style) {
        *style = self;
    }
}

// Support for closures as modifiers
impl<F> StyleModifier for F where F: FnOnce(&mut Style) {
    fn apply(self, style: &mut Style) {
        self(style);
    }
}

// Support for multiple modifiers via Tuples
impl<A: StyleModifier, B: StyleModifier> StyleModifier for (A, B) {
    fn apply(self, style: &mut Style) {
        self.0.apply(style);
        self.1.apply(style);
    }
}

impl<A: StyleModifier, B: StyleModifier, C: StyleModifier> StyleModifier for (A, B, C) {
    fn apply(self, style: &mut Style) {
        self.0.apply(style);
        self.1.apply(style);
        self.2.apply(style);
    }
}

// --- Functional Style Atomic Units ---

pub fn p(val: impl IntoSpacing) -> impl StyleModifier {
    move |s: &mut Style| { s.padding = val.into_spacing(); }
}

pub fn m(val: impl IntoSpacing) -> impl StyleModifier {
    move |s: &mut Style| { s.margin = val.into_spacing(); }
}

pub fn bg(color: impl Into<Color>) -> impl StyleModifier {
    let color = color.into();
    move |s: &mut Style| { s.background.color = Some(color.clone()); }
}

pub fn w(val: f32) -> impl StyleModifier {
    move |s: &mut Style| { s.sizing.width = Some(val); }
}

pub fn h(val: f32) -> impl StyleModifier {
    move |s: &mut Style| { s.sizing.height = Some(val); }
}

pub fn rounded(val: f32) -> impl StyleModifier {
    move |s: &mut Style| { s.rounding = crate::utils::border::Rounding::all(val); }
}
