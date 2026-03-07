#[derive(Clone, Debug, PartialEq, Default)]
pub enum TableLayout { #[default] Auto, Fixed }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BorderCollapse { #[default] Separate, Collapse }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Table {
    pub layout: TableLayout,
    pub border_collapse: BorderCollapse,
    pub border_spacing: [f32; 2],
    pub caption_side: String, // "top", "bottom"
}
