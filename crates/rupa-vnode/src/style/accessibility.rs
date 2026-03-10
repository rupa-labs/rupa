use crate::a11y::SemanticRole;

#[derive(Clone, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Accessibility {
    pub role: SemanticRole,
    pub label: Option<String>,
    pub value: Option<String>,
    pub disabled: bool,
    pub focused: bool,
}
