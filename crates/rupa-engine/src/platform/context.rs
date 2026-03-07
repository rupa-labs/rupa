use std::sync::{Arc, RwLock};
use crate::scene::SceneCore;
use rupa_core::{vector::Vec2, state::Signal};
use rupa_core::component::Component;
use crate::platform::app::AppMetadata;
use rupa_core::events::CursorIcon;
use rupa_core::error::{DiagnosticCenter, Error};

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
pub struct PlatformCore {
    pub metadata: AppMetadata,
    pub root: Option<Box<dyn Component>>,
    pub scene: SceneCore,
    pub viewport: Signal<Vec2>,
    pub cursor_pos: Vec2,
    pub requested_cursor: CursorIcon,
    pub pointer_capture: Option<String>,
    pub focused_id: Option<String>,
    pub a11y_enabled: bool,
    pub debug: bool,
    pub diagnostic_center: Option<DiagnosticCenter>,
    // Allows plugins or debuggers to intercept events
    pub event_listeners: Vec<Arc<dyn Fn(&crate::platform::events::InputEvent) + Send + Sync>>,
}

pub type SharedPlatformCore = Arc<RwLock<PlatformCore>>;

impl PlatformCore {
    pub fn new(metadata: AppMetadata, root: Option<Box<dyn Component>>) -> Self {
        Self {
            metadata,
            root,
            scene: SceneCore::new(),
            viewport: Signal::new(Vec2::zero()),
            cursor_pos: Vec2::zero(),
            requested_cursor: CursorIcon::Default,
            pointer_capture: None,
            focused_id: None,
            a11y_enabled: true,
            debug: false,
            diagnostic_center: None,
            event_listeners: Vec::new(),
        }
    }

    /// Reports an error to the diagnostic center if it exists.
    pub fn report_error(&self, error: Error) {
        if let Some(ref dc) = self.diagnostic_center {
            dc.report(error);
        }
    }

    /// Adds a component to the global overlay stack of the root Body.
    pub fn add_overlay(&self, component: Box<dyn Component>) {
        if let Some(ref root) = self.root {
            // Downcasting to Body to access its logic
            if let Some(body) = root.as_any().downcast_ref::<crate::core::body::Body>() {
                body.logic.add_overlay(component);
                body.mark_dirty();
            }
        }
    }

    /// Common logic to compute the layout tree via SceneCore.
    pub fn compute_layout(&mut self, measurer: &dyn crate::renderer::TextMeasurer, width: f32, height: f32) -> Option<crate::scene::SceneNode> {
        if let Some(ref root) = self.root {
            return Some(self.scene.resolve(root.as_ref(), measurer, width, height));
        }
        None
    }
}
