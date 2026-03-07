#[derive(Clone, Debug, PartialEq, Default)]
pub enum ForcedColorAdjust { #[default] Auto, None }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Role { #[default] None, Button, Link, Checkbox, Menu, MenuItem, Navigation, Tab, TabPanel, Status, Alert }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Accessibility {
    pub forced_color_adjust: ForcedColorAdjust,
    pub role: crate::support::Role,
    pub label: Option<String>,
    pub description: Option<String>,
    pub labelled_by: Option<String>,
    pub described_by: Option<String>,
    pub hidden: bool,
    pub visually_hidden: bool, // Helper: Visually Hidden
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_accessibility_defaults() {
        let acc = Accessibility::default();
        assert_eq!(acc.role, Role::None);
        assert_eq!(acc.hidden, false);
    }

    #[test]
    fn test_accessibility_role() {
        let mut acc = Accessibility::default();
        acc.role = Role::Button;
        acc.label = Some("Save".to_string());
        assert_eq!(acc.role, Role::Button);
        assert_eq!(acc.label.unwrap(), "Save");
    }
}
