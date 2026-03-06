#[derive(Clone, Debug, PartialEq, Default)]
pub enum FlexDirection { #[default] Row, Col, RowReverse, ColReverse }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FlexWrap { #[default] NoWrap, Wrap, WrapReverse }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum AlignItems { #[default] Stretch, FlexStart, FlexEnd, Center, Baseline }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum JustifyContent { #[default] FlexStart, FlexEnd, Center, SpaceBetween, SpaceAround, SpaceEvenly }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Flex {
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Option<f32>,
    pub align_items: Option<AlignItems>,
    pub justify_content: Option<JustifyContent>,
    pub gap: Option<f32>,
}
