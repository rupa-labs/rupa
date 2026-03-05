// Snippet updates for existing files
// crates/rupaui/src/utils/layout.rs -> add pub clearfix: bool to Layout
// crates/rupaui/src/utils/effects.rs -> add pub focus_ring: bool to Effects
// crates/rupaui/src/utils/interactivity.rs -> add pub stretched_link: bool to Interactivity
// crates/rupaui/src/utils/accessibility.rs -> add pub visually_hidden: bool to Accessibility

// I will overwrite layout.rs first to include clearfix
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Display { #[default] Flex, Grid, Block, Inline, InlineBlock, Contents, None }
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Position { #[default] Static, Relative, Absolute, Fixed, Sticky }
pub use super::layout::{Float, Clear, Overflow, Visibility, ObjectFit, BoxSizing, BreakMode, BreakInside, BoxDecorationBreak, Isolation, OverscrollBehavior};

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
    pub clearfix: bool, // Helper: Clearfix
}
