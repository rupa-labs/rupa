//! # Rupa CLI 🛠️
//!
//! Aesthetic command-line tooling and project initializer for the Rupa Framework. 
//! This crate provides the developer experience (DX) utilities to scaffold, 
//! build, and manage Rupa projects across different showrooms.

pub mod commands;
pub mod templates;
pub mod bundler;
pub mod config;

/// The main entry point for the CLI logic.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    commands::handle().await
}
