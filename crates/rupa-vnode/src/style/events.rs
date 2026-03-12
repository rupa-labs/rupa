use rupa_base::Vec2;
use serde::{Serialize, Deserialize};

/// A platform-agnostic UI event triggered by user interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIEvent {
    Pointer(PointerEvent),
    Key(KeyEvent),
    Focus(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerEvent {
    pub pos: Vec2,
    pub delta: Vec2,
    pub button: Option<PointerButton>,
    pub modifiers: Modifiers,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PointerButton {
    Primary,
    Secondary,
    Middle,
    Other(u16),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEvent {
    pub key: String,
    pub pressed: bool,
    pub modifiers: Modifiers,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl UIEvent {
    pub fn is_pointer(&self) -> bool { matches!(self, UIEvent::Pointer(_)) }
    pub fn is_key(&self) -> bool { matches!(self, UIEvent::Key(_)) }
}
