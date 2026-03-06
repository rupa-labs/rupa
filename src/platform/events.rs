use std::sync::Arc;
use crate::utils::vector::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventPhase {
    Capture,
    Target,
    Bubble,
}

pub struct UIEvent {
    pub phase: EventPhase,
    pub consumed: bool,
}

impl UIEvent {
    pub fn new() -> Self {
        Self { phase: EventPhase::Target, consumed: false }
    }
    pub fn stop_propagation(&mut self) {
        self.consumed = true;
    }
}

pub type ClickCallback = Arc<dyn Fn(&mut UIEvent) + Send + Sync>;
pub type ScrollCallback = Arc<dyn Fn(&mut UIEvent, f32) + Send + Sync>;
pub type DragCallback = Arc<dyn Fn(&mut UIEvent, Vec2) + Send + Sync>;

#[derive(Default, Clone)]
pub struct EventListeners {
    pub on_click: Option<ClickCallback>,
    pub on_scroll: Option<ScrollCallback>,
    pub on_drag: Option<DragCallback>,
}

impl std::fmt::Debug for EventListeners {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventListeners").finish()
    }
}

impl PartialEq for EventListeners {
    fn eq(&self, _other: &Self) -> bool { true }
}
