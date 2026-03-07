use rupa_core::vnode::VNode; use rupa_core::component::Component;
use rupa_styling::Style;
use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn hover(modifier: impl StyleModifier) -> impl StyleModifier {
    move |s: &mut Style| {
        let mut sub_style = Style::default();
        modifier.apply(&mut sub_style);
        s.variants.insert("hover".to_string(), sub_style);
    }
}

pub fn active(modifier: impl StyleModifier) -> impl StyleModifier {
    move |s: &mut Style| {
        let mut sub_style = Style::default();
        modifier.apply(&mut sub_style);
        s.variants.insert("active".to_string(), sub_style);
    }
}

pub fn focus(modifier: impl StyleModifier) -> impl StyleModifier {
    move |s: &mut Style| {
        let mut sub_style = Style::default();
        modifier.apply(&mut sub_style);
        s.variants.insert("focus".to_string(), sub_style);
    }
}

pub fn group_hover(modifier: impl StyleModifier) -> impl StyleModifier {
    move |s: &mut Style| {
        let mut sub_style = Style::default();
        modifier.apply(&mut sub_style);
        s.group_hover = Some(Box::new(sub_style));
    }
}

pub fn is_group() -> impl StyleModifier {
    |s: &mut Style| {
        s.is_group = true;
    }
}

// --- Chaining API ---

pub trait ChainedState: Stylable {
    fn hover(self, m: impl StyleModifier) -> Self { self.style(hover(m)) }
    fn active(self, m: impl StyleModifier) -> Self { self.style(active(m)) }
    fn focus(self, m: impl StyleModifier) -> Self { self.style(focus(m)) }
    fn group_hover(self, m: impl StyleModifier) -> Self { self.style(group_hover(m)) }
    fn is_group(self) -> Self { self.style(is_group()) }
}

impl<T: Stylable> ChainedState for T {}
