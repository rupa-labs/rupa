/// A platform-agnostic representation of an accessibility role.
/// This follows Dependency Inversion by wrapping external roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticRole {
    Button,
    Checkbox,
    TextInput,
    Slider,
    Heading,
    Label,
    List,
    ListItem,
    Window,
    GenericContainer,
}

/// Metadata for accessibility services (Screen Readers).
#[derive(Debug, Clone, Default)]
pub struct AccessibilityNode {
    pub role: Option<SemanticRole>,
    pub label: Option<String>,
    pub description: Option<String>,
    pub value: Option<String>,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
}
