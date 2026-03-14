use rupa_core::{Component, VNode, Id};
use rupa_vnode::{Style};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

// --- SVG ---

pub struct Svg {
    pub id: String,
    pub content: String,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Svg {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            id: Id::next().to_string(),
            content: content.into(),
            style: Arc::new(RwLock::new(Style::default())),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for Svg {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("svg")
            .with_style(self.get_style().read().unwrap().clone())
            .with_attr("content", self.content.clone())
            .with_key(self.id.clone())
    }
}

impl Stylable for Svg {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}

// --- ICON ---

pub struct Icon {
    pub id: String,
    pub name: String,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Icon {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Id::next().to_string(),
            name: name.into(),
            style: Arc::new(RwLock::new(Style::default())),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for Icon {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("icon")
            .with_style(self.get_style().read().unwrap().clone())
            .with_attr("name", self.name.clone())
            .with_key(self.id.clone())
    }
}

impl Stylable for Icon {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
