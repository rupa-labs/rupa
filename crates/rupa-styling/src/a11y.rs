use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AccessibilityNode {
    pub role: SemanticRole,
    pub label: Option<String>,
    pub value: Option<String>,
    pub disabled: bool,
    pub focused: bool,
}
