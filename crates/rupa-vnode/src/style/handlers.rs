use std::sync::Arc;
use crate::style::events::UIEvent;

pub type EventHandler = Arc<dyn Fn(UIEvent) + Send + Sync>;

/// A collection of event callbacks for a VElement.
/// These are skipped during serialization because they contain logic.
#[derive(Clone, Default)]
pub struct EventHandlers {
    pub on_click: Option<EventHandler>,
    pub on_hover: Option<EventHandler>,
    pub on_key_down: Option<EventHandler>,
    pub on_focus: Option<EventHandler>,
    pub on_blur: Option<EventHandler>,
    pub on_scroll: Option<EventHandler>,
}

impl std::fmt::Debug for EventHandlers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventHandlers")
            .field("on_click", &self.on_click.is_some())
            .field("on_hover", &self.on_hover.is_some())
            .finish()
    }
}

impl PartialEq for EventHandlers {
    fn eq(&self, _other: &Self) -> bool {
        true // Handlers are considered equal for VNode diffing purposes
    }
}
