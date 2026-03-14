use rupa_vnode::{Style, Unit};
use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn w(val: impl Into<Unit>) -> impl StyleModifier {
    let unit = val.into();
    move |s: &mut Style| s.sizing.width = Some(unit.clone())
}

pub fn h(impl_val: impl Into<Unit>) -> impl StyleModifier {
    let unit = impl_val.into();
    move |s: &mut Style| s.sizing.height = Some(unit.clone())
}

pub fn w_full() -> impl StyleModifier {
    move |s: &mut Style| s.sizing.width = Some(Unit::Absolute(-1.0))
}

pub fn h_full() -> impl StyleModifier {
    move |s: &mut Style| s.sizing.height = Some(Unit::Absolute(-1.0))
}

pub fn size(width: impl Into<Unit>, height: impl Into<Unit>) -> impl StyleModifier {
    let w_unit = width.into();
    let h_unit = height.into();
    move |s: &mut Style| {
        s.sizing.width = Some(w_unit.clone());
        s.sizing.height = Some(h_unit.clone());
    }
}

// --- Chaining API ---

pub trait ChainedSizing: Stylable {
    fn w(self, val: impl Into<Unit>) -> Self { self.style(w(val)) }
    fn h(self, val: impl Into<Unit>) -> Self { self.style(h(val)) }
    fn w_full(self) -> Self { self.style(w_full()) }
    fn h_full(self) -> Self { self.style(h_full()) }
    fn size(self, width: impl Into<Unit>, height: impl Into<Unit>) -> Self { self.style(size(width, height)) }
}

impl<T: Stylable> ChainedSizing for T {}
