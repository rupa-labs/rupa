use crate::style::utilities::spacing::Spacing;

pub enum Cursor { Default, Pointer, Text, Move, Wait, Help, NotAllowed, Crosshair }

pub struct Interactivity {
    pub cursor: Cursor,
    pub pointer_events: bool,
    pub user_select: bool,
    pub touch_action: String,
    pub scroll_margin: Spacing,
    pub scroll_padding: Spacing,
}

impl Default for Interactivity {
    fn default() -> Self {
        Self {
            cursor: Cursor::Default,
            pointer_events: true,
            user_select: true,
            touch_action: "auto".to_string(),
            scroll_margin: Spacing::default(),
            scroll_padding: Spacing::default(),
        }
    }
}
