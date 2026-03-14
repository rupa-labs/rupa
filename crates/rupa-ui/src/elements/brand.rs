use rupa_core::{Component, VNode, Id};
use rupa_vnode::{Style, Theme};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

// --- BRAND ---

/// A specialized wrapper for brand identities.
pub struct Brand {
    pub id: String,
    pub name: String,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Brand {
    pub fn new(name: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        
        Self {
            id: Id::next().to_string(),
            name: name.into(),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Stylable for Brand {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}

impl Component for Brand {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("brand")
            .with_style(self.style.read().unwrap().clone())
            .with_attr("name", self.name.clone())
            .with_child(VNode::text(self.name.clone()))
            .with_key(self.id.clone())
    }
}
