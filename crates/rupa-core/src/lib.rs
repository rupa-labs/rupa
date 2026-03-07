pub mod vector;
pub mod error;
pub mod component;
pub mod events;
pub mod types;
pub mod renderer;
pub mod scene;
pub mod a11y;
pub mod view;
pub mod style_data;
pub mod motion;

// Re-export Atomic Pieces
pub use rupa_signals as signals;
pub use rupa_signals::{Signal, Memo, Effect, Readable};
pub use rupa_signals::id::generate_id;

// Internal Core Modules
pub mod spacing;
pub mod border;
pub mod background;
pub mod color;
pub mod layout;
pub mod flex;
pub mod grid;
pub mod sizing;
pub mod typography;
pub mod interactivity;
pub mod effects;
pub mod filters;
pub mod accessibility;
pub mod attributes;

pub use vector::Vec2;
pub use error::Error;
pub use component::Component;
pub use events::{InputEvent, KeyCode, PointerButton, ButtonState, CursorIcon, UIEvent, Modifiers, EventListeners};
pub use types::{TextAlign, FontWeight, Breakpoint};
pub use renderer::{Renderer, TextMeasurer, RenderCore};
pub use scene::SceneNode;
pub use a11y::{AccessibilityNode, SemanticRole};
pub use view::ViewCore;
pub use style_data::Style;
pub use motion::{Motion, Transition, Animation, TimingFunction, Easing};

// Re-export common style types
pub use spacing::Spacing;
pub use border::{Border, Rounding, Outline};
pub use background::Background;
pub use color::Color;
pub use layout::{Layout, Display, Position};
pub use flex::{Flex, FlexDirection, AlignItems, JustifyContent};
pub use grid::Grid;
pub use sizing::Sizing;
pub use typography::TypographyStyle;
pub use interactivity::Interactivity;
pub use effects::Shadow;
pub use filters::Filter;
pub use accessibility::Accessibility;
pub use attributes::Attributes;
