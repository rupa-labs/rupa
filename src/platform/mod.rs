pub mod events;
pub mod dispatcher;
pub mod a11y;
pub mod app;
pub mod context;
pub mod runner;

#[cfg(feature = "gui")]
pub mod desktop;

#[cfg(feature = "tui")]
pub mod terminal;

pub mod web;
pub mod mobile;

pub use self::a11y::{SemanticRole, AccessibilityNode};
pub use self::app::{App, AppMetadata};
pub use self::context::{PlatformCore, SharedPlatformCore, request_redraw, register_redraw_proxy};
pub use self::runner::PlatformEvent;
