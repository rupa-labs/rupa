pub mod events;
pub mod dispatcher;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "tui")]
pub mod tui;

use crate::core::component::Component;
use crate::core::plugin::PluginRegistry;
use crate::scene::SceneCore;
use crate::support::vector::Vec2;
use crate::support::Theme;
use std::error::Error;
use taffy::prelude::NodeId;

/// Common state shared across all platform backends (GUI, TUI, etc).
/// This is an example of "Composition over Inheritance".
pub struct PlatformCore {
    pub app_name: String,
    pub root: Option<Box<dyn Component>>,
    pub scene: SceneCore,
    pub cursor_pos: Vec2,
}

impl PlatformCore {
    pub fn new(app_name: String, root: Option<Box<dyn Component>>) -> Self {
        Self {
            app_name,
            root,
            scene: SceneCore::new(),
            cursor_pos: Vec2::zero(),
        }
    }

    /// Common logic to compute the layout tree via SceneCore.
    pub fn compute_layout(&mut self, width: f32, height: f32) -> Option<NodeId> {
        if let Some(ref root) = self.root {
            return Some(self.scene.resolve(root.as_ref(), width, height));
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
pub enum RupauiEvent {
    RequestRedraw,
}

pub fn request_redraw() {
    #[cfg(feature = "gui")]
    if let Some(proxy) = gui::get_event_proxy() {
        let _ = proxy.send_event(RupauiEvent::RequestRedraw);
    }
}

pub struct App {
    pub name: String,
    pub root: Option<Box<dyn Component>>,
    pub registry: PluginRegistry,
}

impl App {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            root: None,
            registry: PluginRegistry::new(),
        }
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
        let runner = gui::GuiRunner::new(self.name.clone(), self.root);
        runner.run_app();
    }

    #[cfg(feature = "tui")]
    pub fn run_tui(mut self) {
        self.bootstrap();
        let mut runner = tui::TuiRunner::new(self.name.clone(), self.root);
        if let Err(e) = runner.run() {
            eprintln!("TUI Error: {}", e);
        }
    }
}
