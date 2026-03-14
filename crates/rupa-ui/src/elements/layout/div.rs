use rupa_core::{Component, VNode, Id};
use rupa_vnode::Style;
use crate::style::modifiers::Stylable;
use crate::elements::Children;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// A bare generic container with no opinionated styles or behaviors.
pub struct Div<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl<'a> Div<'a> {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            children: Children::new(),
            style: Arc::new(RwLock::new(Style::default())),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }

    pub fn child(mut self, child: impl Component + 'a) -> Self {
        self.children.push(std::boxed::Box::new(child));
        self
    }
}

impl<'a> Component for Div<'a> {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    
    fn render(&self) -> VNode {
        VNode::element("div")
            .with_style(self.get_style().read().unwrap().clone())
            .with_children(self.children.render_all())
            .with_key(self.id.clone())
    }
}

impl<'a> Stylable for Div<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
