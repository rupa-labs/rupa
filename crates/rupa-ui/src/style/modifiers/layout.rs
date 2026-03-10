use rupa_vnode::{Style, Display, Position, FlexDirection, AlignItems, JustifyContent};
use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn flex() -> impl StyleModifier {
    move |s: &mut Style| s.layout.display = Display::Flex
}

pub fn grid() -> impl StyleModifier {
    move |s: &mut Style| s.layout.display = Display::Grid
}

pub fn flex_row() -> impl StyleModifier {
    move |s: &mut Style| {
        s.layout.display = Display::Flex;
        s.flex.flex_direction = rupa_vnode::FlexDirection::Row;
    }
}

pub fn flex_col() -> impl StyleModifier {
    move |s: &mut Style| {
        s.layout.display = Display::Flex;
        s.flex.flex_direction = rupa_vnode::FlexDirection::Col;
    }
}

pub fn items_center() -> impl StyleModifier {
    move |s: &mut Style| s.flex.align_items = Some(AlignItems::Center)
}

pub fn justify_center() -> impl StyleModifier {
    move |s: &mut Style| s.flex.justify_content = Some(JustifyContent::Center)
}

pub fn justify_between() -> impl StyleModifier {
    move |s: &mut Style| s.flex.justify_content = Some(JustifyContent::SpaceBetween)
}

pub fn absolute() -> impl StyleModifier {
    move |s: &mut Style| s.layout.position = rupa_vnode::Position::Absolute
}

pub fn relative() -> impl StyleModifier {
    move |s: &mut Style| s.layout.position = Position::Relative
}

pub fn z(val: i32) -> impl StyleModifier {
    move |s: &mut Style| s.layout.z_index = val
}

// --- Chaining API ---

pub trait ChainedLayout: Stylable {
    fn flex(self) -> Self { self.style(flex()) }
    fn grid(self) -> Self { self.style(grid()) }
    fn flex_row(self) -> Self { self.style(flex_row()) }
    fn flex_col(self) -> Self { self.style(flex_col()) }
    fn items_center(self) -> Self { self.style(items_center()) }
    fn justify_center(self) -> Self { self.style(justify_center()) }
    fn justify_between(self) -> Self { self.style(justify_between()) }
    fn absolute(self) -> Self { self.style(absolute()) }
    fn relative(self) -> Self { self.style(relative()) }
    fn z(self, val: i32) -> Self { self.style(z(val)) }
}

impl<T: Stylable> ChainedLayout for T {}
