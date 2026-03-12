use serde::{Serialize, Deserialize};
use rupa_base::Rect;

/// Semantic roles for UI elements, based on ARIA standards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Window,
    Button,
    CheckBox,
    Heading,
    Link,
    TextInput,
    Slider,
    Label,
    List,
    ListItem,
    Image,
    Tab,
    TabPanel,
    Menu,
    MenuItem,
}

/// A platform-agnostic accessibility node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub role: Role,
    pub label: Option<String>,
    pub value: Option<String>,
    pub bounds: Option<Rect>,
    pub is_focused: bool,
    pub is_hidden: bool,
    pub is_disabled: bool,
}

impl Node {
    pub fn new(role: Role) -> Self {
        Self {
            role,
            label: None,
            value: None,
            bounds: None,
            is_focused: false,
            is_hidden: false,
            is_disabled: false,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}
