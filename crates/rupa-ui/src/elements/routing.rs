use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_styling::{Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
use crate::style::modifiers::base::Stylable;
use std::sync::RwLockWriteGuard;
use std::collections::HashMap;
use once_cell::sync::Lazy;

// --- GLOBAL ROUTER STATE ---

static CURRENT_PATH: Lazy<Signal<String>> = Lazy::new(|| Signal::new("/".to_string()));

pub struct RouterState;
impl RouterState {
    pub fn push(path: impl Into<String>) {
        let p = path.into();
        CURRENT_PATH.set(p.clone());
    }

    pub fn current() -> String {
        CURRENT_PATH.get()
    }
}

// --- ROUTER COMPONENT ---

pub type RouteBuilder = Box<dyn Fn(&str) -> Box<dyn Component> + Send + Sync>;

pub struct RouterLogic {
    pub routes: HashMap<String, RouteBuilder>,
    pub fallback: Option<RouteBuilder>,
}

pub struct RouterView {
    pub core: ViewCore,
}

pub struct Router {
    pub id: String,
    pub logic: RouterLogic,
    pub view: RouterView,
}

impl Router {
    pub fn new() -> Self {
        Self {
            id: format!("router-{}", generate_id()),
            logic: RouterLogic {
                routes: HashMap::new(),
                fallback: None,
            },
            view: RouterView { core: ViewCore::new() },
        }
    }

    pub fn route(mut self, path: impl Into<String>, builder: impl Fn(&str) -> Box<dyn Component> + Send + Sync + 'static) -> Self {
        self.logic.routes.insert(path.into(), Box::new(builder));
        self
    }

    pub fn fallback(mut self, builder: impl Fn(&str) -> Box<dyn Component> + Send + Sync + 'static) -> Self {
        self.logic.fallback = Some(Box::new(builder));
        self
    }

    fn resolve_active_component(&self) -> Box<dyn Component> {
        let current = CURRENT_PATH.get();
        if let Some(builder) = self.logic.routes.get(&current) {
            builder(&current)
        } else if let Some(ref fallback) = self.logic.fallback {
            fallback(&current)
        } else {
            Box::new(crate::primitives::div::Div::new())
        }
    }
}

impl Stylable for Router {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

impl Component for Router {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn render(&self) -> VNode {
        let active = self.resolve_active_component();
        active.render()
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut taffy::prelude::TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<taffy::prelude::NodeId>) -> taffy::prelude::NodeId {
        let active = self.resolve_active_component();
        let node = active.layout(taffy, measurer, parent);
        self.view.core.set_node(SceneNode::from(node));
        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &taffy::prelude::TaffyTree<()>, node: taffy::prelude::NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let active = self.resolve_active_component();
        active.paint(renderer, taffy, node, is_group_hovered, global_pos);
    }
}
