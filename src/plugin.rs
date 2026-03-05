use crate::app::App;

/// The base contract for all Rupaui plugins.
/// Plugins can extend the app with new state, theme presets, or logic.
pub trait Plugin: Send + Sync {
    /// Returns a unique identifier for the plugin.
    fn name(&self) -> &str;

    /// Called during App initialization.
    /// This is where you register theme defaults or custom utilities.
    fn build(&self, app: &mut App);
}

/// A container for managing multiple plugins.
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn add(&mut self, plugin: Box<dyn Plugin>) {
        log::info!("Registering plugin: {}", plugin.name());
        self.plugins.push(plugin);
    }

    pub fn build_all(&self, app: &mut App) {
        for plugin in &self.plugins {
            plugin.build(app);
        }
    }
}
