use rupa_vnode::{Style, Spacing, Scale};
use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn p(val: impl Into<Scale>) -> impl StyleModifier {
    let scale = val.into();
    move |s: &mut Style| s.padding = Spacing::all(scale.value(16.0))
}

pub fn px(val: impl Into<Scale>) -> impl StyleModifier {
    let scale = val.into();
    move |s: &mut Style| {
        let v = scale.value(16.0);
        s.padding.left = v;
        s.padding.right = v;
    }
}

pub fn py(val: impl Into<Scale>) -> impl StyleModifier {
    let scale = val.into();
    move |s: &mut Style| {
        let v = scale.value(16.0);
        s.padding.top = v;
        s.padding.bottom = v;
    }
}

pub fn pt(val: impl Into<Scale>) -> impl StyleModifier {
    let scale = val.into();
    move |s: &mut Style| s.padding.top = scale.value(16.0)
}

pub fn pr(val: impl Into<Scale>) -> impl StyleModifier {
    let scale = val.into();
    move |s: &mut Style| s.padding.right = scale.value(16.0)
}

pub fn pb(val: impl Into<Scale>) -> impl StyleModifier {
    let scale = val.into();
    move |s: &mut Style| s.padding.bottom = scale.value(16.0)
}

pub fn pl(val: impl Into<Scale>) -> impl StyleModifier {
    let scale = val.into();
    move |s: &mut Style| s.padding.left = scale.value(16.0)
}

pub fn gap(val: impl Into<Scale>) -> impl StyleModifier {
    let scale = val.into();
    move |s: &mut Style| s.flex.gap = scale.value(16.0)
}

// --- Chaining API ---

pub trait ChainedSpacing: Stylable {
    fn p(self, val: impl Into<Scale>) -> Self { self.style(p(val)) }
    fn px(self, val: impl Into<Scale>) -> Self { self.style(px(val)) }
    fn py(self, val: impl Into<Scale>) -> Self { self.style(py(val)) }
    fn pt(self, val: impl Into<Scale>) -> Self { self.style(pt(val)) }
    fn pr(self, val: impl Into<Scale>) -> Self { self.style(pr(val)) }
    fn pb(self, val: impl Into<Scale>) -> Self { self.style(pb(val)) }
    fn pl(self, val: impl Into<Scale>) -> Self { self.style(pl(val)) }
    fn gap(self, val: impl Into<Scale>) -> Self { self.style(gap(val)) }
}

impl<T: Stylable> ChainedSpacing for T {}
