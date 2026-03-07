use std::sync::{Arc, RwLock};
use crate::core::component::Component;
use crate::core::plugin::PluginRegistry;
use crate::support::Style;
use crate::support::Theme;
use crate::platform::context::PlatformCore;

#[cfg(feature = "desktop")]
use crate::platform::desktop::DesktopRunner;
#[cfg(feature = "terminal")]
use crate::platform::terminal::TerminalRunner;
#[cfg(feature = "web")]
use crate::platform::web::WebRunner;
#[cfg(feature = "mobile")]
use crate::platform::mobile::MobileRunner;

use crate::platform::runner::PlatformRunner;

#[derive(Debug, Clone)]
pub struct AppMetadata {
    pub title: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub identifier: String, // e.g. "com.reasnov.myapp"
}

pub struct App {
    pub metadata: AppMetadata,
    pub root: Option<Box<dyn Component>>,
    pub registry: PluginRegistry,
    pub body_style: Style,
    pub debug: bool,
    pub error_handler: Option<Arc<dyn Fn(crate::support::error::RupauiError) + Send + Sync>>,
    initial_listeners: Vec<Arc<dyn Fn(&crate::platform::events::InputEvent) + Send + Sync>>,
}

impl App {
    pub fn new(title: impl Into<String>) -> Self {
        let t = title.into();
        Self {
            metadata: AppMetadata {
                title: t.clone(),
                version: "0.1.0".into(),
                description: String::new(),
                author: "".into(),
                identifier: format!("org.rupaui.{}", t.to_lowercase().replace(" ", "-")),
            },
            root: None,
            registry: PluginRegistry::new(),
            body_style: Style::default(),
            debug: false,
            error_handler: None,
            initial_listeners: Vec::new(),
        }
    }

    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    pub fn on_error(mut self, handler: impl Fn(crate::support::error::RupauiError) + Send + Sync + 'static) -> Self {
        self.error_handler = Some(Arc::new(handler));
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.metadata.title = title.into();
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.metadata.version = version.into();
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.metadata.description = desc.into();
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.metadata.author = author.into();
        self
    }

    pub fn identifier(mut self, id: impl Into<String>) -> Self {
        self.metadata.identifier = id.into();
        self
    }

    /// Style the implicit root 'Body' element (Viewport).
    pub fn style(mut self, modifier: impl crate::style::modifiers::base::StyleModifier) -> Self {
        modifier.apply(&mut self.body_style);
        self
    }

    /// Register a global event listener, typically called by plugins during bootstrap.
    pub fn add_event_listener(&mut self, listener: impl Fn(&crate::platform::events::InputEvent) + Send + Sync + 'static) {
        self.initial_listeners.push(Arc::new(listener));
    }

    pub fn root(mut self, component: impl Component + 'static) -> Self {
        self.root = Some(Box::new(component));
        self
    }

    fn bootstrap(&mut self) {
        let _ = Theme::current();
        let registry = std::mem::replace(&mut self.registry, PluginRegistry::new());
        registry.build_all(self);
    }

    fn prepare_root(&mut self) -> Box<dyn Component> {
        // Automatically wrap user root into an implicit internal Body primitive
        let body = crate::core::body::Body::new(self.body_style.clone(), self.root.take());
        Box::new(body)
    }

    #[cfg(feature = "desktop")]
    pub fn run(mut self) {
        self.bootstrap();
        let final_root = self.prepare_root();
        let mut core_data = PlatformCore::new(self.metadata.clone(), Some(final_root));
        core_data.event_listeners = std::mem::take(&mut self.initial_listeners);
        core_data.debug = self.debug;
        
        if let Some(ref handler) = self.error_handler {
            core_data.diagnostic_center = Some(crate::support::error::DiagnosticCenter {
                handler: Arc::clone(handler),
            });
        }
        
        let core = Arc::new(RwLock::new(core_data));
        let runner = DesktopRunner::new(core);
        if let Err(e) = runner.run() {
            eprintln!("Desktop Error: {}", e);
        }
    }

    #[cfg(feature = "terminal")]
    pub fn run_terminal(mut self) {
        self.bootstrap();
        let final_root = self.prepare_root();
        let mut core_data = PlatformCore::new(self.metadata.clone(), Some(final_root));
        core_data.event_listeners = std::mem::take(&mut self.initial_listeners);
        core_data.debug = self.debug;

        if let Some(ref handler) = self.error_handler {
            core_data.diagnostic_center = Some(crate::support::error::DiagnosticCenter {
                handler: Arc::clone(handler),
            });
        }

        let core = Arc::new(RwLock::new(core_data));
        let runner = TerminalRunner::new(core);
        if let Err(e) = runner.run() {
            eprintln!("Terminal Error: {}", e);
        }
    }

    #[cfg(feature = "web")]
    pub fn run_web(mut self, canvas_id: impl Into<String>) {
        self.bootstrap();
        let final_root = self.prepare_root();
        let mut core_data = PlatformCore::new(self.metadata.clone(), Some(final_root));
        core_data.event_listeners = std::mem::take(&mut self.initial_listeners);
        core_data.debug = self.debug;
        
        let core = Arc::new(RwLock::new(core_data));
        let runner = WebRunner::new(core, canvas_id);
        if let Err(e) = runner.run() {
            eprintln!("Web Error: {}", e);
        }
    }

    #[cfg(feature = "mobile")]
    pub fn run_mobile(mut self) {
        self.bootstrap();
        let final_root = self.prepare_root();
        let mut core_data = PlatformCore::new(self.metadata.clone(), Some(final_root));
        core_data.event_listeners = std::mem::take(&mut self.initial_listeners);
        core_data.debug = self.debug;
        
        let core = Arc::new(RwLock::new(core_data));
        let runner = MobileRunner::new(core);
        if let Err(e) = runner.run() {
            eprintln!("Mobile Error: {}", e);
        }
    }
}
