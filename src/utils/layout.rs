#[derive(Clone, Debug, PartialEq, Default)]
pub enum Display {
    #[default]
    Flex,
    Grid,
    Block,
    Inline,
    InlineBlock,
    Contents,
    None,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Position {
    #[default]
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Float { #[default] None, Left, Right }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Clear { #[default] None, Left, Right, Both }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Overflow { #[default] Visible, Hidden, Clip, Scroll, Auto }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Visibility { #[default] Visible, Hidden, Collapse }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ObjectFit { #[default] Fill, Contain, Cover, None, ScaleDown }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BoxSizing { #[default] BorderBox, ContentBox }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BreakMode { #[default] Auto, Avoid, Always, All, AvoidPage, Page, Left, Right, Column }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BreakInside { #[default] Auto, Avoid, AvoidPage, AvoidColumn }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BoxDecorationBreak { #[default] Slice, Clone }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Isolation { #[default] Auto, Isolate }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum OverscrollBehavior { #[default] Auto, Contain, None }

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Layout {
    pub display: Display,
    pub position: Position,
    pub float: Float,
    pub clear: Clear,
    pub box_sizing: BoxSizing,
    pub isolation: Isolation,
    pub visibility: Visibility,
    pub overflow: Overflow,
    pub overscroll: OverscrollBehavior,
    pub aspect_ratio: Option<f32>,
    pub columns: Option<u16>,
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
    pub z_index: Option<i32>,
}
