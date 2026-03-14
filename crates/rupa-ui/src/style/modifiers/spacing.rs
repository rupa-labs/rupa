use rupa_vnode::{Style, Spacing, Unit};
use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn p(val: impl Into<Unit>) -> impl StyleModifier {
    let unit = val.into();
    move |s: &mut Style| s.padding = Spacing::all(unit.clone())
}

pub fn px(val: impl Into<Unit>) -> impl StyleModifier {
    let unit = val.into();
    move |s: &mut Style| {
        s.padding.left = unit.clone();
        s.padding.right = unit.clone();
    }
}

pub fn py(val: impl Into<Unit>) -> impl StyleModifier {
    let unit = val.into();
    move |s: &mut Style| {
        s.padding.top = unit.clone();
        s.padding.bottom = unit.clone();
    }
}

pub fn pt(val: impl Into<Unit>) -> impl StyleModifier {
    let unit = val.into();
    move |s: &mut Style| s.padding.top = unit.clone()
}

pub fn pr(val: impl Into<Unit>) -> impl StyleModifier {
    let unit = val.into();
    move |s: &mut Style| s.padding.right = unit.clone()
}

pub fn pb(val: impl Into<Unit>) -> impl StyleModifier {
    let unit = val.into();
    move |s: &mut Style| s.padding.bottom = unit.clone()
}

pub fn pl(val: impl Into<Unit>) -> impl StyleModifier {
    let unit = val.into();
    move |s: &mut Style| s.padding.left = unit.clone()
}

// --- Chaining API ---

pub trait ChainedSpacing: Stylable {
    fn p(self, val: impl Into<Unit>) -> Self { self.style(p(val)) }
    fn px(self, val: impl Into<Unit>) -> Self { self.style(px(val)) }
    fn py(self, val: impl Into<Unit>) -> Self { self.style(py(val)) }
    fn pt(self, val: impl Into<Unit>) -> Self { self.style(pt(val)) }
    fn pr(self, val: impl Into<Unit>) -> Self { self.style(pr(val)) }
    fn pb(self, val: impl Into<Unit>) -> Self { self.style(pb(val)) }
    fn pl(self, val: impl Into<Unit>) -> Self { self.style(pl(val)) }
}

impl<T: Stylable> ChainedSpacing for T {}
