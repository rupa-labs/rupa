use crate::support::vector::Vec2;

/// Standardized input events that are platform-agnostic.
/// Layer 1 (HAL) is responsible for converting native events into these types.
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    // Pointer Events (Mouse, Touch, Pen)
    PointerMove { position: Vec2 },
    PointerButton { button: PointerButton, state: ButtonState },
    PointerScroll { delta: Vec2 },
    
    // Keyboard Events
    Key { key: KeyCode, state: ButtonState, modifiers: Modifiers },
    
    // System Events
    Resize { size: Vec2, scale_factor: f64 },
    Focus(bool),
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointerButton {
    Primary,   // Left click
    Secondary, // Right click
    Auxiliary, // Middle click
    Extra(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
