#[derive(Clone, Debug, PartialEq, Default)]
pub enum FlexDirection { #[default] Row, Column, RowReverse, ColumnReverse }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FlexWrap { #[default] NoWrap, Wrap, WrapReverse }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum AlignItems { #[default] Start, Center, End, Stretch, Baseline }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum JustifyContent { #[default] Start, Center, End, SpaceBetween, SpaceAround, SpaceEvenly }

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Flex {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
    pub grow: f32,
    pub shrink: f32,
    pub basis: Option<f32>,
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
    pub order: i32,
}
