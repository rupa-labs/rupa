#[derive(Clone, Debug, PartialEq, Default)]
pub enum Display { #[default] Flex, Grid, Block, Inline, InlineBlock, Contents, None }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Position { #[default] Static, Relative, Absolute, Fixed, Sticky }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Overflow { #[default] Visible, Hidden, Scroll, Auto }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layout {
    pub display: Display,
    pub position: Position,
    pub overflow_x: Overflow,
    pub overflow_y: Overflow,
    pub z_index: Option<i32>,
    pub columns: Option<u16>,
}
