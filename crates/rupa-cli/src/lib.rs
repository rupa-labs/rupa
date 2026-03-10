pub mod commands;
pub mod templates;
pub mod bundler;
pub mod config;

/// The main entry point for the CLI logic.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    commands::handle().await
}
