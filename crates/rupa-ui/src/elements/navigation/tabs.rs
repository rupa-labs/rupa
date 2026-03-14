use rupa_core::{Component, VNode, Id};
use rupa_vnode::{Style};
use crate::style::modifiers::Stylable;
use crate::elements::Children;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// # Component: Tabs 📑
pub struct Tabs<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl<'a> Tabs<'a> {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            children: Children::new(),
            style: Arc::new(RwLock::new(Style::default())),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl<'a> Component for Tabs<'a> {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    
    fn render(&self) -> VNode {
        VNode::element("tabs")
            .with_style(self.style.read().unwrap().clone())
            .with_children(self.children.render_all())
            .with_key(self.id.clone())
    }
}

impl<'a> Stylable for Tabs<'a> {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
