#[derive(Clone, Debug, PartialEq, Default)]
pub enum ForcedColorAdjust { #[default] Auto, None }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Role { #[default] None, Button, Link, Checkbox, Menu, MenuItem, Navigation, Tab, TabPanel, Status, Alert }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Accessibility {
    pub forced_color_adjust: ForcedColorAdjust,
    pub role: Role,
    pub label: Option<String>,
    pub description: Option<String>,
    pub labelled_by: Option<String>,
    pub described_by: Option<String>,
    pub hidden: bool,
    pub visually_hidden: bool, // Helper: Visually Hidden
}
