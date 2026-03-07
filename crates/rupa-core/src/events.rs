use rupa_support::Vec2;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

/// The UIEvent passed to components during dispatch.
/// Contains rich context about the user interaction.
#[derive(Clone)]
pub struct UIEvent {
    pub consumed: bool,
    pub local_pos: Vec2,
    pub modifiers: Modifiers,
    pub button: Option<PointerButton>,
    pub button_state: Option<ButtonState>,
    pub capture_request: Option<bool>,
    pub focus_request: Option<bool>, // Some(true) to request focus, Some(false) to blur
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
    pub fn capture_pointer(&mut self) { self.capture_request = Some(true); }
    pub fn release_pointer(&mut self) { self.capture_request = Some(false); }
    pub fn request_focus(&mut self) { self.focus_request = Some(true); }
    pub fn blur(&mut self) { self.focus_request = Some(false); }
}

/// Standardized input events that are platform-agnostic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputEvent {
    // Pointer Events (Mouse, Touch, Pen)
    PointerMove { position: Vec2 },
    PointerButton { button: PointerButton, state: ButtonState },
    PointerScroll { delta: Vec2 },
    
    // Keyboard Events
    Key { key: KeyCode, state: ButtonState, modifiers: Modifiers },
    Ime(String),
    
    // System Events
    Resize { size: Vec2, scale_factor: f64 },
    SafeArea { top: f32, right: f32, bottom: f32, left: f32 },
    Focus(bool),
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PointerButton {
    #[default]
    Primary,   // Left click
    Secondary, // Right click
    Auxiliary, // Middle click
    Extra(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    Char(char),
    Enter,
    Escape,
    Tab,
    Backspace,
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    Home, End, PageUp, PageDown,
    Delete, Insert,
    F(u8),
    Unknown,
}

/// A container for common event callbacks used by components.
#[derive(Default)]
pub struct EventListeners {
    pub on_click: Option<Arc<dyn Fn(&mut UIEvent) + Send + Sync>>,
    pub on_release: Option<Arc<dyn Fn(&mut UIEvent) + Send + Sync>>,
    pub on_scroll: Option<Arc<dyn Fn(&mut UIEvent, f32) + Send + Sync>>,
    pub on_drag: Option<Arc<dyn Fn(&mut UIEvent, Vec2) + Send + Sync>>,
    pub on_key: Option<Arc<dyn Fn(&mut UIEvent, KeyCode) + Send + Sync>>,
    pub on_text: Option<Arc<dyn Fn(&mut UIEvent, &str) + Send + Sync>>,
}
