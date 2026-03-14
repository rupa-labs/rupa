use rupa_core::{Component, VNode, Id};
use rupa_vnode::{Style, Position};
use crate::style::modifiers::Stylable;
use crate::elements::Children;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// # Component: Tooltip 💬
pub struct Tooltip<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl<'a> Tooltip<'a> {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.layout.position = Position::Absolute;
        Self {
            id: Id::next().to_string(),
            children: Children::new(),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }

    pub fn child(mut self, child: impl Component + 'a) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl<'a> Component for Tooltip<'a> {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    
    fn render(&self) -> VNode {
        VNode::element("tooltip")
            .with_style(self.style.read().unwrap().clone())
            .with_children(self.children.render_all())
            .with_key(self.id.clone())
    }
}

impl<'a> Stylable for Tooltip<'a> {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
