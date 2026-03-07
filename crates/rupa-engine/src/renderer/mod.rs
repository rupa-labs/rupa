pub mod core;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "tui")]
pub mod tui;

pub use self::core::{RenderCore, TextMeasurer, Renderer};

#[cfg(feature = "gui")]
pub use gui::renderer::Renderer as GuiRenderer;
#[cfg(feature = "tui")]
pub use tui::TuiRenderer;
