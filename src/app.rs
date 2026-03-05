use crate::plugin::{Plugin, PluginRegistry};
use crate::utils::Theme;

/// The primary entry point for a Rupaui application.
/// Manages the application lifecycle and plugin registry.
pub struct App {
    pub name: String,
    pub registry: PluginRegistry,
}

impl App {
    /// Creates a new application instance.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            registry: PluginRegistry::new(),
        }
    }

    /// Adds a plugin to the application.
    pub fn add_plugin(mut self, plugin: impl Plugin + 'static) -> Self {
        self.registry.add(Box::new(plugin));
        self
    }

    /// Starts the application, building all plugins and initializing the theme.
    pub fn run(mut self) {
        log::info!("Starting Rupaui Application: {}", self.name);
        
        // 1. Initialize DNA Visual from Default Artisan Theme
        let _ = Theme::current();

        // 2. Build all registered plugins
        self.registry.build_all(&mut self);

        // 3. Start the UI event loop (to be integrated with Winit/WGPU later)
        log::debug!("Application loop running...");
    }
}
