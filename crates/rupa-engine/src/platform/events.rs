use rupa_core::vector::Vec2;
use std::sync::Arc;
use rupa_core::events::UIEvent;

/// Standardized input events that are platform-agnostic.
/// Layer 1 (Platform Integration) is responsible for converting native events into these types.
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    // Pointer Events (Mouse, Touch, Pen)
    PointerMove { position: Vec2 },
    PointerButton { button: PointerButton, state: ButtonState },
    PointerScroll { delta: Vec2 },
    
    // Keyboard Events
    Key { key: KeyCode, state: ButtonState, modifiers: Modifiers },
    Ime(String), // Input Method Editor for international/complex text input
    
    // System Events
    Resize { size: Vec2, scale_factor: f64 },
    SafeArea { top: f32, right: f32, bottom: f32, left: f32 },
    Focus(bool),
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorIcon {
    #[default]
    Default,
    Pointer,
    Text,
    Grab,
    Grabbing,
    NotAllowed,
    Wait,
    Crosshair,
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
