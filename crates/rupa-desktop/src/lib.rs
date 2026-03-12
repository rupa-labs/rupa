//! # Rupa Desktop 🖥️
//!
//! The Desktop Showroom for the Rupa Framework. This crate provides the 
//! **Adapters & Infrastructure** (Tier 3) to render Rupa applications 
//! natively using GPU acceleration (WGPU) and OS-level windowing (Winit).

pub mod renderer;
pub mod runner;

pub use rupa_core as core;
pub use rupa_ui as ui;
pub use rupa_engine as engine;
pub use rupa_signals as signals;
pub use rupa_assets as assets;
pub use rupa_motion as motion;

/// The standard prelude for building Rupa Desktop applications.
pub mod prelude {
    pub use rupa_core::{Component, Signal, Memo, Effect, Readable};
    pub use rupa_ui::elements::button::*;
    pub use rupa_ui::elements::text::*;
    pub use rupa_ui::elements::layout::*;
    pub use rupa_ui::elements::forms::*;
    pub use rupa_ui::primitives::div::Div;
    pub use rupa_ui::primitives::flex::Flex;
    pub use rupa_ui::primitives::grid::Grid;
    pub use rupa_ui::style::*;
    pub use rupa_engine::App;
    pub use crate::runner::DesktopRunner;
}
