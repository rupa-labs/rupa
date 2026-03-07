use rupa_core::style_data::Style;
use rupa_core::layout::Display;
use rupa_core::flex::{FlexDirection, AlignItems, JustifyContent};
use rupa_core::scale::Scale;
use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn flex() -> impl StyleModifier { |s: &mut Style| s.layout.display = Display::Flex }
pub fn col() -> impl StyleModifier { |s: &mut Style| s.flex.flex_direction = FlexDirection::Col }
pub fn row() -> impl StyleModifier { |s: &mut Style| s.flex.flex_direction = FlexDirection::Row }
pub fn gap(val: f32) -> impl StyleModifier { move |s: &mut Style| s.flex.gap = Some(val) }
pub fn items_center() -> impl StyleModifier { |s: &mut Style| s.flex.align_items = Some(AlignItems::Center) }
pub fn justify_center() -> impl StyleModifier { |s: &mut Style| s.flex.justify_content = Some(JustifyContent::Center) }

pub fn gap_scale(sc: Scale) -> impl StyleModifier { move |s: &mut Style| s.flex.gap = Some(sc.value(16.0)) }

// --- Chaining API ---

pub trait ChainedLayout: Stylable {
    fn flex(self) -> Self { self.style(flex()) }
    fn col(self) -> Self { self.style(col()) }
    fn row(self) -> Self { self.style(row()) }
    fn gap(self, val: f32) -> Self { self.style(gap(val)) }
    fn items_center(self) -> Self { self.style(items_center()) }
    fn justify_center(self) -> Self { self.style(justify_center()) }
}

impl<T: Stylable> ChainedLayout for T {}
