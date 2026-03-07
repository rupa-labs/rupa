use rupa_core::vnode::VNode; use rupa_core::component::Component;
use rupa_core::component::Component;
use rupa_core::view::ViewCore;
use rupa_core::{Style, generate_id, Vec2, state::Signal};
use rupa_core::renderer::{Renderer, TextMeasurer};
use rupa_core::scene::SceneNode;
use crate::style::modifiers::base::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};
use std::collections::HashMap;
use once_cell::sync::Lazy;

// --- GLOBAL ROUTER STATE ---

static CURRENT_PATH: Lazy<Signal<String>> = Lazy::new(|| Signal::new("/".to_string()));

pub struct RouterState;
impl RouterState {
    pub fn push(path: impl Into<String>) {
        let p = path.into();
        CURRENT_PATH.set(p.clone());
        
        #[cfg(target_arch = "wasm32")]
        {
            let _ = crate::platform::web::infra::WebInfra::push_state(&p);
        }
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
            // Default empty component if no match and no fallback
            Box::new(crate::primitives::div::Div::new())
        }
    }
}

impl Stylable for Router {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() }
}

impl Component for Router {
    fn render(&self) -> VNode { VNode::Empty }
    fn id(&self) -> &str { &self.id }
    
    fn children(&self) -> Vec<&dyn Component> {
        // Since active component is dynamic, we don't expose it here easily for hit-testing 
        // without a more complex scene integration. But for layout, we build it.
        vec![] 
    }

    fn as_any(&self) -> &dyn std::any::Any { self }

    fn layout(&self, taffy: &mut taffy::prelude::TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<taffy::prelude::NodeId>) -> taffy::prelude::NodeId {
        let active = self.resolve_active_component();
        let node = active.layout(taffy, measurer, parent);
        self.view.core.set_node(SceneNode::from(node));
        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &taffy::prelude::TaffyTree<()>, node: taffy::prelude::NodeId, is_group_hovered: bool, global_pos: Vec2) {
        // Router itself is transparent logic, it just delegatess paint to matched route
        // This is handled via the layout node being identical.
        // But for clarity, we'll re-resolve or cache the active component.
        let active = self.resolve_active_component();
        active.paint(renderer, taffy, node, is_group_hovered, global_pos);
    }
}
