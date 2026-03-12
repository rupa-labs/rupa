//! # Rupa Docs 📚
//!
//! Interactive documentation builder and viewer for the Rupa Framework. 
//! This crate acts as an internal Tooling application to showcase 
//! the framework's capabilities across all rendering targets.

use rupa::prelude::*;

/// The main entry point for building the Rupa Documentation UI.
/// This component is designed to be executed across Desktop (GPU), 
/// Terminal (TUI), and Web (DOM) showrooms.
pub fn docs_app() -> impl Component {
    VStack::new()
        .style((p(20.0), gap(10.0), bg_primary()))
        .child(Text::new("Rupa Framework Documentation")
            .style(font_bold()))
        .child(Text::new("The polyglot, full-stack UI framework for Rust artisans."))
        .child(Button::new("Get Started")
            .variant(Variant::Secondary)
            .on_click(|_| log::info!("Documentation: Get Started clicked!")))
}
