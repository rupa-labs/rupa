use crate::utils::color::Color;
use crate::utils::spacing::Spacing;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Appearance { #[default] Auto, None }
#[derive(Clone, Debug, PartialEq, Default)]
pub enum ColorScheme { #[default] Normal, Light, Dark }
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Cursor { #[default] Default, Pointer, Wait, Text, Move, Help, NotAllowed, Crosshair, Grab, Grabbing, EResize, NResize, NEResize, NWResize, SResize, SEResize, SWResize, WResize }
#[derive(Clone, Debug, PartialEq, Default)]
pub enum PointerEvents { #[default] Auto, None, All, VisiblePainted, VisibleFill, VisibleStroke, Painted, Fill, Stroke }
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Resize { #[default] None, Both, Horizontal, Vertical }
#[derive(Clone, Debug, PartialEq, Default)]
pub enum ScrollBehavior { #[default] Auto, Smooth }
#[derive(Clone, Debug, PartialEq, Default)]
pub enum ScrollSnapAlign { #[default] None, Start, End, Center }
#[derive(Clone, Debug, PartialEq, Default)]
pub enum ScrollSnapStop { #[default] Normal, Always }
#[derive(Clone, Debug, PartialEq, Default)]
pub enum ScrollSnapType { #[default] None, X, Y, Both, Mandatory, Proximity }
#[derive(Clone, Debug, PartialEq, Default)]
pub enum TouchAction { #[default] Auto, None, PanX, PanY, PinchZoom, Manipulation }
#[derive(Clone, Debug, PartialEq, Default)]
pub enum UserSelect { #[default] Auto, None, Text, All }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Interactivity {
    pub accent_color: Option<Color>,
    pub appearance: Appearance,
    pub caret_color: Option<Color>,
    pub color_scheme: ColorScheme,
    pub cursor: Cursor,
    pub field_sizing: String,
    pub pointer_events: PointerEvents,
    pub resize: Resize,
    pub scroll_behavior: ScrollBehavior,
    pub scroll_margin: Spacing,
    pub scroll_padding: Spacing,
    pub scroll_snap_align: ScrollSnapAlign,
    pub scroll_snap_stop: ScrollSnapStop,
    pub scroll_snap_type: ScrollSnapType,
    pub touch_action: TouchAction,
    pub user_select: UserSelect,
    pub will_change: Vec<String>,
    pub stretched_link: bool, // Helper: Stretched Link
}
