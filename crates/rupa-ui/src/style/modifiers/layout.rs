use rupa_vnode::{Style, Display, Position, AlignItems, JustifyContent, Unit};
use rupa_vnode::style::grid::{GridTrack, GridLine};

use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn flex() -> impl StyleModifier {
    move |s: &mut Style| s.layout.display = Display::Flex
}

pub fn grid() -> impl StyleModifier {
    move |s: &mut Style| s.layout.display = Display::Grid
}

// Grid Container Modifiers
pub fn cols(n: u16) -> impl StyleModifier {
    move |s: &mut Style| {
        s.layout.display = Display::Grid;
        s.grid.template_columns = (0..n).map(|_| GridTrack::Fr(1.0)).collect();
    }
}

pub fn rows(n: u16) -> impl StyleModifier {
    move |s: &mut Style| {
        s.layout.display = Display::Grid;
        s.grid.template_rows = (0..n).map(|_| GridTrack::Fr(1.0)).collect();
    }
}

pub fn grid_gap(val: impl Into<Unit>) -> impl StyleModifier {
    let unit = val.into();
    move |s: &mut Style| {
        let v = unit.resolve(4.0); // TUI base 1.0, GUI base 4.0/8.0? Using 4.0 as default for now
        s.grid.column_gap = v;
        s.grid.row_gap = v;
    }
}

// Grid Item Modifiers
pub fn col_span(n: u16) -> impl StyleModifier {
    move |s: &mut Style| s.grid_item.column_end = GridLine::Span(n)
}

pub fn row_span(n: u16) -> impl StyleModifier {
    move |s: &mut Style| s.grid_item.row_end = GridLine::Span(n)
}

pub fn col_start(n: i16) -> impl StyleModifier {
    move |s: &mut Style| s.grid_item.column_start = GridLine::Index(n)
}

pub fn row_start(n: i16) -> impl StyleModifier {
    move |s: &mut Style| s.grid_item.row_start = GridLine::Index(n)
}

pub fn span_full() -> impl StyleModifier {
    move |s: &mut Style| {
        s.grid_item.column_start = GridLine::Index(1);
        s.grid_item.column_end = GridLine::Index(-1);
    }
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

pub fn justify_start() -> impl StyleModifier {
    move |s: &mut Style| s.flex.justify_content = Some(JustifyContent::FlexStart)
}

pub fn justify_center() -> impl StyleModifier {
    move |s: &mut Style| s.flex.justify_content = Some(JustifyContent::Center)
}

pub fn justify_end() -> impl StyleModifier {
    move |s: &mut Style| s.flex.justify_content = Some(JustifyContent::FlexEnd)
}

pub fn justify_between() -> impl StyleModifier {
    move |s: &mut Style| s.flex.justify_content = Some(JustifyContent::SpaceBetween)
}

pub fn justify_around() -> impl StyleModifier {
    move |s: &mut Style| s.flex.justify_content = Some(JustifyContent::SpaceAround)
}

pub fn items_start() -> impl StyleModifier {
    move |s: &mut Style| s.flex.align_items = Some(AlignItems::FlexStart)
}

pub fn items_center() -> impl StyleModifier {
    move |s: &mut Style| s.flex.align_items = Some(AlignItems::Center)
}

pub fn items_end() -> impl StyleModifier {
    move |s: &mut Style| s.flex.align_items = Some(AlignItems::FlexEnd)
}

pub fn items_stretch() -> impl StyleModifier {
    move |s: &mut Style| s.flex.align_items = Some(AlignItems::Stretch)
}

pub fn gap(val: impl Into<Unit>) -> impl StyleModifier {
    let unit = val.into();
    move |s: &mut Style| {
        let v = unit.resolve(4.0);
        s.flex.gap = Some(v);
        s.grid.column_gap = v;
        s.grid.row_gap = v;
    }
}

pub fn flex_grow(val: f32) -> impl StyleModifier {
    move |s: &mut Style| s.flex.flex_grow = val
}

pub fn flex_shrink(val: f32) -> impl StyleModifier {
    move |s: &mut Style| s.flex.flex_shrink = val
}

pub fn wrap() -> impl StyleModifier {
    move |s: &mut Style| s.flex.flex_wrap = rupa_vnode::style::flex::FlexWrap::Wrap
}

pub fn no_wrap() -> impl StyleModifier {
    move |s: &mut Style| s.flex.flex_wrap = rupa_vnode::style::flex::FlexWrap::NoWrap
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
    fn cols(self, n: u16) -> Self { self.style(cols(n)) }
    fn rows(self, n: u16) -> Self { self.style(rows(n)) }
    fn grid_gap(self, val: impl Into<Unit>) -> Self { self.style(grid_gap(val)) }
    
    fn col_span(self, n: u16) -> Self { self.style(col_span(n)) }
    fn row_span(self, n: u16) -> Self { self.style(row_span(n)) }
    fn col_start(self, n: i16) -> Self { self.style(col_start(n)) }
    fn row_start(self, n: i16) -> Self { self.style(row_start(n)) }
    fn span_full(self) -> Self { self.style(span_full()) }

    fn flex_row(self) -> Self { self.style(flex_row()) }
    fn flex_col(self) -> Self { self.style(flex_col()) }
    
    fn justify_start(self) -> Self { self.style(justify_start()) }
    fn justify_center(self) -> Self { self.style(justify_center()) }
    fn justify_end(self) -> Self { self.style(justify_end()) }
    fn justify_between(self) -> Self { self.style(justify_between()) }
    fn justify_around(self) -> Self { self.style(justify_around()) }

    fn items_start(self) -> Self { self.style(items_start()) }
    fn items_center(self) -> Self { self.style(items_center()) }
    fn items_end(self) -> Self { self.style(items_end()) }
    fn items_stretch(self) -> Self { self.style(items_stretch()) }

    fn gap(self, val: impl Into<Unit>) -> Self { self.style(gap(val)) }
    fn flex_grow(self, val: f32) -> Self { self.style(flex_grow(val)) }
    fn flex_shrink(self, val: f32) -> Self { self.style(flex_shrink(val)) }
    fn wrap(self) -> Self { self.style(wrap()) }
    fn no_wrap(self) -> Self { self.style(no_wrap()) }

    fn absolute(self) -> Self { self.style(absolute()) }
    fn relative(self) -> Self { self.style(relative()) }
    fn z(self, val: i32) -> Self { self.style(z(val)) }
}

impl<T: Stylable> ChainedLayout for T {}
