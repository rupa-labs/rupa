use std::sync::{Arc, RwLock};
use rupa_core::component::Component;
use crate::plugin::PluginRegistry;
use rupa_vnode::{Style, Theme};
use crate::platform::context::PlatformCore;
use rupa_core::{Signal, Vec2, Error};
use rupa_core::events::InputEvent;

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
pub enum IconSource {
    Path(String),
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    Browser,
    Standalone,
    MinimalUi,
    Fullscreen,
}

#[derive(Debug, Clone)]
pub struct AppMetadata {
    pub title: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub identifier: String,
    pub icon: Option<IconSource>,
    pub theme_color: Option<[f32; 4]>,
    pub background_color: Option<[f32; 4]>,
    pub display_mode: DisplayMode,
}

pub struct App {
    pub metadata: AppMetadata,
    pub root: Option<Box<dyn Component>>,
    pub registry: PluginRegistry,
    pub body_style: Style,
    pub debug: bool,
    pub initial_overlays: Vec<Box<dyn Component>>,
    pub error_handler: Option<Arc<dyn Fn(Error) + Send + Sync>>,
    initial_listeners: Vec<Arc<dyn Fn(&InputEvent) + Send + Sync>>,
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
                identifier: format!("org.rupa.{}", t.to_lowercase().replace(" ", "-")),
                icon: None,
                theme_color: None,
                background_color: None,
                display_mode: DisplayMode::Browser,
            },
            root: None,
            registry: PluginRegistry::new(),
            body_style: Style::default(),
            debug: false,
            initial_overlays: Vec::new(),
            error_handler: None,
            initial_listeners: Vec::new(),
        }
    }

    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    pub fn on_error(mut self, handler: impl Fn(Error) + Send + Sync + 'static) -> Self {
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

    pub fn icon(mut self, source: IconSource) -> Self {
        self.metadata.icon = Some(source);
        self
    }

    pub fn theme_color(mut self, rgba: [f32; 4]) -> Self {
        self.metadata.theme_color = Some(rgba);
        self
    }

    pub fn background_color(mut self, rgba: [f32; 4]) -> Self {
        self.metadata.background_color = Some(rgba);
        self
    }

    pub fn display_mode(mut self, mode: DisplayMode) -> Self {
        self.metadata.display_mode = mode;
        self
    }

    /// Style the implicit root 'Body' element.
    pub fn style(mut self, modifier: impl Fn(&mut Style)) -> Self {
        modifier(&mut self.body_style);
        self
    }

    pub fn add_event_listener(&mut self, listener: impl Fn(&InputEvent) + Send + Sync + 'static) {
        self.initial_listeners.push(Arc::new(listener));
    }

    pub fn root(mut self, component: impl Component + 'static) -> Self {
        self.root = Some(Box::new(component));
        self
    }

    pub fn overlay(mut self, component: impl Component + 'static) -> Self {
        self.initial_overlays.push(Box::new(component));
        self
    }

    fn bootstrap(&mut self) {
        let _ = Theme::current();
        let registry = std::mem::replace(&mut self.registry, PluginRegistry::new());
        registry.build_all(self);
    }

    fn prepare_root(&mut self, viewport: Signal<Vec2>) -> Box<dyn Component> {
        let mut body = rupa_ui::Body::new();
        *body.view.core.style() = self.body_style.clone();
        // Overlays and children will be handled differently in the new architecture
        // For now, if we have a root, we should add it
        if let Some(r) = self.root.take() {
            body = body.child(r);
        }
        Box::new(body)
    }

    #[cfg(feature = "desktop")]
    pub fn run(mut self) {
        self.bootstrap();
        let mut core_data = PlatformCore::new(self.metadata.clone(), None);
        let final_root = self.prepare_root(core_data.viewport.clone());
        core_data.root = Some(final_root);
        core_data.event_listeners = std::mem::take(&mut self.initial_listeners);
        core_data.debug = self.debug;
        
        let core = Arc::new(RwLock::new(core_data));
        let runner = DesktopRunner::new(Arc::clone(&core));
        if let Err(e) = runner.run() {
            log::error!("Desktop Error: {}", e);
        }
    }

    #[cfg(feature = "terminal")]
    pub fn run_terminal(mut self) {
        self.bootstrap();
        let mut core_data = PlatformCore::new(self.metadata.clone(), None);
        let final_root = self.prepare_root(core_data.viewport.clone());
        core_data.root = Some(final_root);
        core_data.event_listeners = std::mem::take(&mut self.initial_listeners);
        core_data.debug = self.debug;

        let core = Arc::new(RwLock::new(core_data));
        let runner = TerminalRunner::new(Arc::clone(&core));
        if let Err(e) = runner.run() {
            log::error!("Terminal Error: {}", e);
        }
    }
}
