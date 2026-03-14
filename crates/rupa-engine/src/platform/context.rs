use std::sync::{Arc, RwLock};
use rupa_core::{Vec2, Signal};
use rupa_core::events::InputEvent; 
use rupa_core::CursorIcon;
use rupa_core::action::BoxedHandler;
use rupa_core::scene::SceneCore;
use rupa_core::scene::layout::LayoutMode;
use crate::platform::app::AppMetadata;
use crate::element_tree::ElementTree;
use std::collections::HashMap;

/// # Platform Core 🏛️
/// 
/// The shared state and internal systems for a Rupa application instance.
/// It orchestrates layout, input, and the virtual component tree.
pub struct PlatformCore {
    pub metadata: AppMetadata,
    pub viewport: Signal<Vec2>,
    pub element_tree: ElementTree,
    pub scene: SceneCore,
    pub cursor_pos: Vec2,
    pub requested_cursor: CursorIcon,
    pub pointer_capture: Option<String>,
    pub focused_id: Option<String>,
    pub hovered_path: Vec<String>,
    pub action_handlers: HashMap<String, Box<dyn BoxedHandler>>,
    pub event_listeners: Vec<Arc<dyn Fn(&InputEvent) + Send + Sync>>,
    pub debug: bool,
}

impl PlatformCore {
    pub fn new(metadata: AppMetadata, mode: LayoutMode) -> Self {
        Self {
            metadata,
            viewport: Signal::new(Vec2::zero()),
            element_tree: ElementTree::new(),
            scene: SceneCore::new(rupa_core::scene::layout::LayoutEngine::new(
                // We'll pass a default solver here, or let the app configure it later
                Box::new(rupa_core::scene::layout::TaffySolver::new()),
                mode
            )),
            cursor_pos: Vec2::zero(),
            requested_cursor: CursorIcon::Default,
            pointer_capture: None,
            focused_id: None,
            hovered_path: Vec::new(),
            action_handlers: HashMap::new(),
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
