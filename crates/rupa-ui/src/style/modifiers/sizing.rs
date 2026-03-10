use rupa_vnode::Style;
use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn w(val: f32) -> impl StyleModifier {
    move |s: &mut Style| s.sizing.width = Some(val)
}

pub fn h(val: f32) -> impl StyleModifier {
    move |s: &mut Style| s.sizing.height = Some(val)
}

pub fn w_full() -> impl StyleModifier {
    move |s: &mut Style| s.sizing.width = Some(100.0)
}

pub fn h_full() -> impl StyleModifier {
    move |s: &mut Style| s.sizing.height = Some(100.0)
}

pub fn size(width: f32, height: f32) -> impl StyleModifier {
    move |s: &mut Style| {
        s.sizing.width = Some(width);
        s.sizing.height = Some(height);
    }
}

// --- Chaining API ---

pub trait ChainedSizing: Stylable {
    fn w(self, val: f32) -> Self { self.style(w(val)) }
    fn h(self, val: f32) -> Self { self.style(h(val)) }
    fn w_full(self) -> Self { self.style(w_full()) }
    fn h_full(self) -> Self { self.style(h_full()) }
    fn size(self, width: f32, height: f32) -> Self { self.style(size(width, height)) }
}

impl<T: Stylable> ChainedSizing for T {}
