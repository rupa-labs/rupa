use rupa_signals::CursorIcon;

#[derive(Clone, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct Interactivity {
    pub cursor: CursorIcon,
    pub pointer_events: bool,
    pub user_select: bool,
}
