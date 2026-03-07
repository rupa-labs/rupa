use winit::keyboard::{Key, NamedKey};
use crate::platform::events::KeyCode;

/// Helper to map winit keys to Rupaui KeyCode.
/// This is isolated to keep the main GUI runner focused on orchestration.
pub fn map_key(event: &winit::event::KeyEvent) -> KeyCode {
    match &event.logical_key {
        Key::Named(named) => match named {
            NamedKey::Enter => KeyCode::Enter,
            NamedKey::Escape => KeyCode::Escape,
            NamedKey::Tab => KeyCode::Tab,
            NamedKey::Backspace => KeyCode::Backspace,
            NamedKey::ArrowUp => KeyCode::ArrowUp,
            NamedKey::ArrowDown => KeyCode::ArrowDown,
            NamedKey::ArrowLeft => KeyCode::ArrowLeft,
            NamedKey::ArrowRight => KeyCode::ArrowRight,
            NamedKey::Home => KeyCode::Home,
            NamedKey::End => KeyCode::End,
            NamedKey::PageUp => KeyCode::PageUp,
            NamedKey::PageDown => KeyCode::PageDown,
            NamedKey::Delete => KeyCode::Delete,
            NamedKey::Insert => KeyCode::Insert,
            NamedKey::F1 => KeyCode::F(1),
            NamedKey::F2 => KeyCode::F(2),
            NamedKey::F3 => KeyCode::F(3),
            NamedKey::F4 => KeyCode::F(4),
            NamedKey::F5 => KeyCode::F(5),
            NamedKey::F6 => KeyCode::F(6),
            NamedKey::F7 => KeyCode::F(7),
            NamedKey::F8 => KeyCode::F(8),
            NamedKey::F9 => KeyCode::F(9),
            NamedKey::F10 => KeyCode::F(10),
            NamedKey::F11 => KeyCode::F(11),
            NamedKey::F12 => KeyCode::F(12),
            _ => KeyCode::Unknown,
        },
        Key::Character(s) => {
            if let Some(c) = s.chars().next() {
                KeyCode::Char(c)
            } else {
                KeyCode::Unknown
            }
        }
        _ => KeyCode::Unknown,
    }
}
