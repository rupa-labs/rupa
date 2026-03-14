use rupa_signals::{Signal, Memo};
use once_cell::sync::Lazy;
use std::sync::Arc;

/// The global state of user interactions across the entire application.
pub struct InteractionState {
    pub focused_id: Signal<Option<String>>,
    pub hovered_id: Signal<Option<String>>,
    pub active_id: Signal<Option<String>>, // For Pressed/Active state
}

pub static INTERACTION: Lazy<Arc<InteractionState>> = Lazy::new(|| {
    Arc::new(InteractionState {
        focused_id: Signal::new(None),
        hovered_id: Signal::new(None),
        active_id: Signal::new(None),
    })
});

impl InteractionState {
    pub fn global() -> Arc<Self> {
        INTERACTION.clone()
    }
}

// --- Interaction Hooks ---

/// Returns a reactive boolean indicating if the given ID is currently focused.
pub fn use_focus(id: impl Into<String>) -> Memo<bool> {
    let target = id.into();
    let state = InteractionState::global();
    Memo::new(move || {
        state.focused_id.get() == Some(target.clone())
    })
}

/// Returns a reactive boolean indicating if the given ID is currently hovered.
pub fn use_hover(id: impl Into<String>) -> Memo<bool> {
    let target = id.into();
    let state = InteractionState::global();
    Memo::new(move || {
        state.hovered_id.get() == Some(target.clone())
    })
}

/// Returns a reactive boolean indicating if the given ID is currently active (pressed).
pub fn use_active(id: impl Into<String>) -> Memo<bool> {
    let target = id.into();
    let state = InteractionState::global();
    Memo::new(move || {
        state.active_id.get() == Some(target.clone())
    })
}
