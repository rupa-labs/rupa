use rupa_base::Vec2;
use serde::{Serialize, Deserialize};

/// The UIEvent passed to components during dispatch.
/// Contains rich context about the user interaction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UIEvent {
    pub consumed: bool,
    pub local_pos: Vec2,
    pub modifiers: Modifiers,
    pub button: Option<PointerButton>,
    pub button_state: Option<ButtonState>,
    pub capture_request: Option<bool>,
    pub focus_request: Option<bool>, 
}

impl UIEvent {
    pub fn new(local_pos: Vec2) -> Self {
        Self { 
            consumed: false, 
            local_pos,
            modifiers: Modifiers::default(),
            button: None,
            button_state: None,
            capture_request: None,
            focus_request: None,
        }
    }

    pub fn with_context(mut self, modifiers: Modifiers, button: Option<PointerButton>, state: Option<ButtonState>) -> Self {
        self.modifiers = modifiers;
        self.button = button;
        self.button_state = state;
        self
    }

    pub fn consume(&mut self) { self.consumed = true; }
    pub fn stop_propagation(&mut self) { self.consumed = true; }
    pub fn capture_pointer(&mut self) { self.capture_request = Some(true); }
    pub fn release_pointer(&mut self) { self.capture_request = Some(false); }
    pub fn request_focus(&mut self) { self.focus_request = Some(true); }
    pub fn blur(&mut self) { self.focus_request = Some(false); }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PointerButton {
    #[default] Primary, Secondary, Middle, Extra(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ButtonState {
    Pressed, Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}
