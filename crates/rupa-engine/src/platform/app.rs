use std::sync::Arc;
use rupa_core::component::ComponentPtr;
use rupa_core::action::{Action, Handler, GenericHandler};
use rupa_core::scene::layout::LayoutMode;
use crate::platform::context::{PlatformCore, SharedPlatformCore};
use crate::plugin::{Plugin, PluginRegistry};

#[derive(Clone)]
pub struct AppMetadata {
    pub title: String,
    pub version: String,
    pub author: String,
}

/// # Rupa App 🚀
/// 
/// The primary orchestrator for a Rupa application. 
/// It manages metadata, plugins, and the core platform execution.
pub struct App {
    pub core: SharedPlatformCore,
    pub plugins: PluginRegistry,
}

impl App {
    /// Creates a new application instance. 
    /// Defaults to [LayoutMode::Pixel] (GUI).
    pub fn new(title: impl Into<String>) -> Self {
        Self::with_mode(title, LayoutMode::Pixel)
    }

    /// Creates a new application instance with a specific [LayoutMode].
    pub fn with_mode(title: impl Into<String>, mode: LayoutMode) -> Self {
        let metadata = AppMetadata {
            title: title.into(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            author: "Rupa Artisan".to_string(),
        };

        Self {
            core: Arc::new(std::sync::RwLock::new(PlatformCore::new(metadata, mode))),
            plugins: PluginRegistry::new(),
        }
    }

    /// Sets the root component of the application.
    pub fn root(self, component: ComponentPtr) -> Self {
        {
            let mut core = self.core.write().unwrap();
            core.element_tree.set_root(component);
        }
        self
    }

    /// Enables or disables debug mode.
    pub fn debug(self, enabled: bool) -> Self {
        {
            let mut core = self.core.write().unwrap();
            core.debug = enabled;
        }
        self
    }

    /// Registers a plugin to extend the application's capabilities. 
    pub fn plugin(mut self, plugin: impl Plugin + 'static) -> Self {
        self.plugins.add(Box::new(plugin));
        self
    }

    /// Registers a global action handler.
    pub fn action<A: Action + 'static>(self, name: impl Into<String>, handler: impl Handler<A> + 'static) -> Self {
        {
            let mut core = self.core.write().unwrap();
            core.action_handlers.insert(name.into(), Box::new(GenericHandler::new(handler)));
        }
        self
    }

    /// Starts the application execution loop. 
    /// This will trigger the build of all registered plugins.
    pub fn run(mut self) {
        log::info!("Starting Rupa App: {}", self.core.read().unwrap().metadata.title);
        
        let registry = std::mem::replace(&mut self.plugins, PluginRegistry::new());
        registry.build_all(&mut self);
        
        if self.handle_cli_actions() {
            return;
        }

        log::warn!("No platform runner attached. Use a Showroom Adapter to run the app.");
    }

    pub fn handle_cli_actions(&self) -> bool {
        let args: Vec<String> = std::env::args().collect();
        if let Some(action_idx) = args.iter().position(|a| a == "--rupa-action") {
            if let Some(action_name) = args.get(action_idx + 1) {
                let payload = if let Some(p_idx) = args.iter().position(|a| a == "--rupa-payload") {
                    args.get(p_idx + 1).cloned().unwrap_or_else(|| "{}".to_string())
                } else {
                    "{}".to_string()
                };

                let core = self.core.read().unwrap();
                if let Some(handler) = core.action_handlers.get(action_name) {
                    let _ = handler.handle_json(&payload);
                }
                return true;
            }
        }
        false
    }
}
