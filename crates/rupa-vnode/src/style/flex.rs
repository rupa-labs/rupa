use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexDirection {
    #[default]
    Row,
    Col,
    RowReverse,
    ColReverse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlignItems {
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Flex {
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub align_items: Option<AlignItems>,
    pub justify_content: Option<JustifyContent>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Option<f32>,
    pub gap: Option<f32>,
}
