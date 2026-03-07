pub mod events;
pub mod dispatcher;
pub mod a11y;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "tui")]
pub mod tui;

pub use self::a11y::{SemanticRole, AccessibilityNode};

use crate::core::component::Component;
use crate::core::plugin::PluginRegistry;
use crate::scene::{SceneCore, SceneNode};
use crate::support::vector::Vec2;
use crate::support::Theme;
use std::error::Error;
use std::sync::{Arc, RwLock};

static GLOBAL_REDRAW_PROXY: RwLock<Option<Box<dyn Fn() + Send + Sync>>> = RwLock::new(None);

pub fn register_redraw_proxy(proxy: impl Fn() + Send + Sync + 'static) {
    if let Ok(mut write_guard) = GLOBAL_REDRAW_PROXY.write() {
        *write_guard = Some(Box::new(proxy));
    }
}

pub fn request_redraw() {
    if let Ok(read_guard) = GLOBAL_REDRAW_PROXY.read() {
        if let Some(proxy) = read_guard.as_ref() {
            (proxy)();
        }
    }
}

/// Common state shared across all platform backends (GUI, TUI, etc).
/// This is wrapped in Arc<RwLock> for thread-safety.
pub struct PlatformCore {
    pub app_name: String,
    pub root: Option<Box<dyn Component>>,
    pub scene: SceneCore,
    pub cursor_pos: Vec2,
    pub pointer_capture: Option<String>,
    pub focused_id: Option<String>,
    pub a11y_enabled: bool,
    // Allows plugins or debuggers to intercept events
    pub event_listeners: Vec<Arc<dyn Fn(&crate::platform::events::InputEvent) + Send + Sync>>,
}

pub type SharedPlatformCore = Arc<RwLock<PlatformCore>>;

impl PlatformCore {
    pub fn new(app_name: String, root: Option<Box<dyn Component>>) -> Self {
        Self {
            app_name,
            root,
            scene: SceneCore::new(),
            cursor_pos: Vec2::zero(),
            pointer_capture: None,
            focused_id: None,
            a11y_enabled: true,
            event_listeners: Vec::new(),
        }
    }

    /// Common logic to compute the layout tree via SceneCore.
    pub fn compute_layout(&mut self, measurer: &dyn crate::renderer::TextMeasurer, width: f32, height: f32) -> Option<SceneNode> {
        if let Some(ref root) = self.root {
            return Some(self.scene.resolve(root.as_ref(), measurer, width, height));
        }
        None
    }
}

pub trait PlatformRunner {
    fn initialize(&mut self) -> Result<(), Box<dyn Error>>;
    fn run(self) -> Result<(), Box<dyn Error>>;
    fn request_redraw(&self);
}

#[derive(Debug)]
pub enum PlatformEvent {
    RequestRedraw,
}

pub struct App {
    pub name: String,
    pub root: Option<Box<dyn Component>>,
    pub registry: PluginRegistry,
    initial_listeners: Vec<Arc<dyn Fn(&crate::platform::events::InputEvent) + Send + Sync>>,
}

impl App {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            root: None,
            registry: PluginRegistry::new(),
            initial_listeners: Vec::new(),
        }
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

    #[cfg(feature = "gui")]
    pub fn run(mut self) {
        self.bootstrap();
        let mut core_data = PlatformCore::new(self.name.clone(), self.root);
        core_data.event_listeners = std::mem::take(&mut self.initial_listeners);
        
        let core = Arc::new(RwLock::new(core_data));
        let runner = gui::GuiRunner::new(core);
        let _ = runner.run_app();
    }

    #[cfg(feature = "tui")]
    pub fn run_tui(mut self) {
        self.bootstrap();
        let mut core_data = PlatformCore::new(self.name.clone(), self.root);
        core_data.event_listeners = std::mem::take(&mut self.initial_listeners);

        let core = Arc::new(RwLock::new(core_data));
        let mut runner = tui::TuiRunner::new(core);
        if let Err(e) = runner.run() {
            eprintln!("TUI Error: {}", e);
        }
    }
}
