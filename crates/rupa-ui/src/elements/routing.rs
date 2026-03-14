use rupa_core::{Component, VNode, Id, Signal};
use rupa_vnode::{Style};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

pub struct RouterState {
    pub current_path: Signal<String>,
}

pub struct Router {
    pub id: String,
    pub state: Arc<RouterState>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Router {
    pub fn new(initial_path: impl Into<String>) -> Self {
        Self {
            id: Id::next().to_string(),
            state: Arc::new(RouterState {
                current_path: Signal::new(initial_path.into()),
            }),
            style: Arc::new(RwLock::new(Style::default())),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for Router {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("router")
            .with_style(self.style.read().unwrap().clone())
            .with_attr("path", self.state.current_path.get())
            .with_key(self.id.clone())
    }
}

impl Stylable for Router {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
