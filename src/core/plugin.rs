use crate::platform::App;

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

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPlugin;
    impl Plugin for MockPlugin {
        fn name(&self) -> &str { "Mock" }
        fn build(&self, _app: &mut App) {
            // Success call
        }
    }

    #[test]
    fn test_plugin_registration() {
        let mut registry = PluginRegistry::new();
        registry.add(Box::new(MockPlugin));
        assert_eq!(registry.plugins.len(), 1);
        assert_eq!(registry.plugins[0].name(), "Mock");
    }
}
