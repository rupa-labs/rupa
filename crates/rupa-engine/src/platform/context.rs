use std::sync::{Arc, RwLock};
use rupa_core::{Component, Vec2, Signal, generate_id, Error};
use rupa_core::events::InputEvent; use rupa_core::CursorIcon;
use crate::platform::app::AppMetadata;
use crate::scene::SceneCore;

pub struct PlatformCore {
    pub metadata: AppMetadata,
    pub viewport: Signal<Vec2>,
    pub root: Option<Box<dyn Component>>,
    pub scene: SceneCore,
    pub cursor_pos: Vec2,
    pub requested_cursor: CursorIcon,
    pub pointer_capture: Option<String>,
    pub focused_id: Option<String>,
    pub event_listeners: Vec<Arc<dyn Fn(&InputEvent) + Send + Sync>>,
    pub debug: bool,
}

impl PlatformCore {
    pub fn new(metadata: AppMetadata, root: Option<Box<dyn Component>>) -> Self {
        Self {
            metadata,
            viewport: Signal::new(Vec2::zero()),
            root,
            scene: SceneCore::new(),
            cursor_pos: Vec2::zero(),
            requested_cursor: CursorIcon::Default,
            pointer_capture: None,
            focused_id: None,
            event_listeners: Vec::new(),
            debug: false,
        }
    }
}

pub type SharedPlatformCore = Arc<RwLock<PlatformCore>>;

pub fn request_redraw() {
    // Platform-specific redraw request logic
}

pub fn register_redraw_proxy(_proxy: Box<dyn Fn() + Send + Sync>) {
    // Platform-specific proxy registration
}
