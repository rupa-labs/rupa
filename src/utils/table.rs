#[derive(Clone, Debug, PartialEq, Default)]
pub enum BorderCollapse { #[default] Separate, Collapse }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TableLayout { #[default] Auto, Fixed }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum CaptionSide { #[default] Top, Bottom }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Table {
    pub border_collapse: BorderCollapse,
    pub border_spacing: (f32, f32), // (horizontal, vertical)
    pub layout: TableLayout,
    pub caption_side: CaptionSide,
}
