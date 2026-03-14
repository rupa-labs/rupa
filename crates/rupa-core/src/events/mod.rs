use rupa_base::Vec2;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

pub use rupa_vnode::style::events::{UIEvent, PointerButton, ButtonState, Modifiers};

/// # Rupa Unified Input Model 🎮
/// 
/// High-level input events that abstract away the hardware (Mouse, Keyboard, Touch) 
/// into semantic "Intents" (Pointer, Focus, System).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputEvent {
    /// --- Pillar 1: Pointer Events (Positional) ---
    /// Represent inputs with spatial coordinates (Mouse, Touch, Pen).
    Pointer {
        position: Vec2,
        action: PointerAction,
        button: Option<PointerButton>,
        modifiers: Modifiers,
    },
    
    /// --- Pillar 2: Focus & Navigation (Logical) ---
    /// Represent logical movement and keyboard interactions (Tab, Arrows, Keys).
    Focus {
        action: FocusAction,
        modifiers: Modifiers,
    },

    /// Raw keyboard input for text processing or specialized hotkeys.
    Key { 
        key: KeyCode, 
        state: ButtonState, 
        modifiers: Modifiers 
    },

    /// --- Pillar 3: System Events ---
    /// Physical environment changes.
    System(SystemEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PointerAction {
    Down,
    Up,
    Move,
    Hover,
    Scroll { delta: Vec2 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FocusAction {
    /// Focus moved into an element.
    Enter,
    /// Focus moved out of an element.
    Leave,
    /// Logical navigation intent (e.g., Tab/Shift+Tab).
    Next,
    Prev,
    /// Directional navigation (e.g., Arrow Keys).
    Direction(NavigationDirection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavigationDirection {
    Up, Down, Left, Right
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SystemEvent {
    Resize { size: Vec2, scale_factor: f64 },
    SafeArea { top: f32, right: f32, bottom: f32, left: f32 },
    WindowFocus(bool),
    Quit,
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

/// A semantic container for intent-based event listeners.
/// Components use these to define THEIR behavior without knowing 
/// if the trigger was a pointer or a key.
#[derive(Default)]
pub struct EventListeners {
    /// High-level intent to "Activate" or "Go" (e.g., Enter key or Click).
    pub on_submit: Option<Arc<dyn Fn(&mut UIEvent) + Send + Sync>>,
    /// High-level intent to "Exit" or "Close" (e.g., Escape key).
    pub on_cancel: Option<Arc<dyn Fn(&mut UIEvent) + Send + Sync>>,
    /// Intent to select or highlight an item (e.g., Hover or Tab-focus).
    pub on_select: Option<Arc<dyn Fn(&mut UIEvent) + Send + Sync>>,
    
    // --- Detailed Listeners ---
    pub on_pointer_down: Option<Arc<dyn Fn(&mut UIEvent) + Send + Sync>>,
    pub on_pointer_up: Option<Arc<dyn Fn(&mut UIEvent) + Send + Sync>>,
    pub on_key_down: Option<Arc<dyn Fn(&mut UIEvent, KeyCode) + Send + Sync>>,
    pub on_text_input: Option<Arc<dyn Fn(String) + Send + Sync>>,
}

pub mod dispatcher;
pub mod interaction;

pub use interaction::*;
