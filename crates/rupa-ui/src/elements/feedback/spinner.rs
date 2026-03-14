use rupa_core::{Component, VNode, Id};
use rupa_vnode::{Style, Theme};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// # Component: Spinner 🌀
pub struct Spinner {
    pub id: String,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Spinner {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: Id::next().to_string(),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for Spinner {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("spinner")
            .with_style(self.style.read().unwrap().clone())
            .with_key(self.id.clone())
    }
}

impl Stylable for Spinner {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
