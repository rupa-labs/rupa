use crate::style::utilities::style::Style;
use crate::style::utilities::color::Color;
use crate::style::utilities::spacing::Spacing;
use crate::style::utilities::border::Rounding;
use crate::style::utilities::scale::Scale;
use crate::core::component::Component;
use std::cell::RefMut;

pub trait StyleModifier {
    fn apply(&self, style: &mut Style);
}

impl<F> StyleModifier for F where F: Fn(&mut Style) {
    fn apply(&self, style: &mut Style) { self(style); }
}

/// The core trait that enables utility-first chaining for any component.
pub trait Stylable: Component + Sized {
    fn style(self, modifier: impl StyleModifier) -> Self {
        {
            let mut style = self.get_style_mut();
            modifier.apply(&mut *style);
        }
        self.mark_dirty();
        self
    }

    /// internal helper to get mutable access to style.
    fn get_style_mut(&self) -> RefMut<'_, Style>;

    // --- Utility Methods (Tailwind-like) ---

    fn p(self, val: f32) -> Self { self.style(move |s: &mut Style| s.padding = Spacing::all(val)) }
    fn px(self, val: f32) -> Self { self.style(move |s: &mut Style| { s.padding.left = val; s.padding.right = val; }) }
    fn py(self, val: f32) -> Self { self.style(move |s: &mut Style| { s.padding.top = val; s.padding.bottom = val; }) }
    
    fn m(self, val: f32) -> Self { self.style(move |s: &mut Style| s.margin = Spacing::all(val)) }
    
    fn bg(self, color: Color) -> Self { self.style(move |s: &mut Style| s.background.color = Some(color.clone())) }
    fn text_color(self, color: Color) -> Self { self.style(move |s: &mut Style| s.typography.color = Some(color.clone())) }
    
    fn rounded(self, val: f32) -> Self { self.style(move |s: &mut Style| s.rounding = Rounding::all(val)) }
    fn rounded_full(self) -> Self { self.style(move |s: &mut Style| s.rounding = Rounding::all(9999.0)) }

    fn w(self, val: f32) -> Self { self.style(move |s: &mut Style| s.sizing.width = Some(val)) }
    fn h(self, val: f32) -> Self { self.style(move |s: &mut Style| s.sizing.height = Some(val)) }
    fn size(self, w: f32, h: f32) -> Self { self.style(move |s: &mut Style| { s.sizing.width = Some(w); s.sizing.height = Some(h); }) }

    fn grow(self) -> Self { self.style(move |s: &mut Style| s.flex.flex_grow = 1.0) }
}
