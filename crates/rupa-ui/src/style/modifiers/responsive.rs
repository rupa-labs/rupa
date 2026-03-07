use rupa_core::vnode::VNode; use rupa_core::component::Component;
use rupa_styling::Style;
use super::base::{StyleModifier, Stylable};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Default)]
pub enum Breakpoint {
    #[default]
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
    Xl5,
    Xl6,
}

impl Breakpoint {
    pub fn min_width(&self) -> f32 {
        match self {
            Breakpoint::Xs => 0.0,
            Breakpoint::Sm => 640.0,
            Breakpoint::Md => 768.0,
            Breakpoint::Lg => 1024.0,
            Breakpoint::Xl => 1280.0,
            Breakpoint::Xl2 => 1536.0,
            Breakpoint::Xl3 => 1920.0,
            Breakpoint::Xl4 => 2560.0,
            Breakpoint::Xl5 => 3840.0,
            Breakpoint::Xl6 => 5120.0,
        }
    }
}

// --- Functional API ---

pub fn breakpoint(bp: Breakpoint, modifier: impl StyleModifier) -> impl StyleModifier {
    move |s: &mut Style| {
        let mut sub_style = Style::default();
        modifier.apply(&mut sub_style);
        s.responsive.insert(bp, Box::new(sub_style));
    }
}

pub fn xs(m: impl StyleModifier) -> impl StyleModifier { breakpoint(Breakpoint::Xs, m) }
pub fn sm(m: impl StyleModifier) -> impl StyleModifier { breakpoint(Breakpoint::Sm, m) }
pub fn md(m: impl StyleModifier) -> impl StyleModifier { breakpoint(Breakpoint::Md, m) }
pub fn lg(m: impl StyleModifier) -> impl StyleModifier { breakpoint(Breakpoint::Lg, m) }
pub fn xl(m: impl StyleModifier) -> impl StyleModifier { breakpoint(Breakpoint::Xl, m) }
pub fn xl2(m: impl StyleModifier) -> impl StyleModifier { breakpoint(Breakpoint::Xl2, m) }
pub fn xl3(m: impl StyleModifier) -> impl StyleModifier { breakpoint(Breakpoint::Xl3, m) }
pub fn xl4(m: impl StyleModifier) -> impl StyleModifier { breakpoint(Breakpoint::Xl4, m) }
pub fn xl5(m: impl StyleModifier) -> impl StyleModifier { breakpoint(Breakpoint::Xl5, m) }
pub fn xl6(m: impl StyleModifier) -> impl StyleModifier { breakpoint(Breakpoint::Xl6, m) }

// --- Chaining API ---

pub trait ChainedResponsive: Stylable {
    fn xs(self, m: impl StyleModifier) -> Self { self.style(xs(m)) }
    fn sm(self, m: impl StyleModifier) -> Self { self.style(sm(m)) }
    fn md(self, m: impl StyleModifier) -> Self { self.style(md(m)) }
    fn lg(self, m: impl StyleModifier) -> Self { self.style(lg(m)) }
    fn xl(self, m: impl StyleModifier) -> Self { self.style(xl(m)) }
    fn xl2(self, m: impl StyleModifier) -> Self { self.style(xl2(m)) }
    fn xl3(self, m: impl StyleModifier) -> Self { self.style(xl3(m)) }
    fn xl4(self, m: impl StyleModifier) -> Self { self.style(xl4(m)) }
    fn xl5(self, m: impl StyleModifier) -> Self { self.style(xl5(m)) }
    fn xl6(self, m: impl StyleModifier) -> Self { self.style(xl6(m)) }
}

impl<T: Stylable> ChainedResponsive for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_order() {
        assert!(Breakpoint::Sm.min_width() > Breakpoint::Xs.min_width());
        assert!(Breakpoint::Md.min_width() > Breakpoint::Sm.min_width());
        assert!(Breakpoint::Xl6.min_width() > Breakpoint::Xl5.min_width());
    }
}
