use crate::spacing::Spacing;
use crate::events::CursorIcon;

pub struct Interactivity {
    pub cursor: CursorIcon,
    pub pointer_events: bool,
    pub user_select: bool,
    pub touch_action: String,
    pub scroll_margin: Spacing,
    pub scroll_padding: Spacing,
}

impl Default for Interactivity {
    fn default() -> Self {
        Self {
            cursor: CursorIcon::Default,
            pointer_events: true,
            user_select: true,
            touch_action: "auto".to_string(),
            scroll_margin: Spacing::default(),
            scroll_padding: Spacing::default(),
        }
    }
}
