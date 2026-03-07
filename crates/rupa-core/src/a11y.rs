#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SemanticRole {
    #[default]
    None,
    Button,
    Link,
    Checkbox,
    TextInput,
    StaticText,
    Image,
    Heading,
    List,
    ListItem,
    Container,
}

#[derive(Debug, Clone, Default)]
pub struct AccessibilityNode {
    pub role: SemanticRole,
    pub label: Option<String>,
    pub value: Option<String>,
    pub disabled: bool,
    pub focused: bool,
}
