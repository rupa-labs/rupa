//! # Rupa Docs 📚
//!
//! Interactive documentation builder and viewer for the Rupa Framework. 
//! This crate acts as an internal Tooling application to showcase 
//! the framework's capabilities across all rendering targets.

use rupa_core::{Component, VNode};
use rupa_ui::prelude::*;

pub struct DocViewer;

impl DocViewer {
    pub fn new() -> Self {
        Self
    }

    pub fn render_page(title: &str, content: &str) -> VNode {
        VStack::new()
            .child(Text::new(title).h1())
            .child(Text::new(content))
            .render()
    }
}

pub fn showcase_component() -> impl Component {
    VStack::new()
        .p(16.0)
        .child(Text::new("Rupa Documentation Showcase").h2())
        .child(Text::new("The polyglot, full-stack UI framework for Rust artisans."))
        .child(Button::new("Get Started")
            .variant(Variant::Secondary)
            .on_click(|_| log::info!("Documentation: Get Started clicked!")))
}
