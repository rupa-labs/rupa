pub mod dispatcher;
pub mod app;
pub mod context;
pub mod runner;

#[cfg(feature = "desktop")]
pub mod desktop;

#[cfg(feature = "terminal")]
pub mod terminal;

pub mod web;
pub mod mobile;

pub use rupa_core::events::*;
pub use rupa_core::a11y::*;
pub use self::app::{App, AppMetadata};
pub use self::context::{PlatformCore, SharedPlatformCore, request_redraw, register_redraw_proxy};
pub use self::runner::PlatformEvent;
