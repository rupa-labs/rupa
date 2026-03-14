use rupa_core::{Component, VNode, Id};
use rupa_vnode::{Style, Display};
use crate::style::modifiers::Stylable;
use crate::elements::Children;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// A container that uses Flexbox layout.
pub struct Flex<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl<'a> Flex<'a> {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.layout.display = Display::Flex;
        Self {
            id: Id::next().to_string(),
            children: Children::new(),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }

    /// Adds a child component to the Flex container.
    pub fn child(mut self, child: impl Component + 'a) -> Self {
        self.children.push(std::boxed::Box::new(child));
        self
    }
}

impl<'a> Component for Flex<'a> {
    fn id(&self) -> &str { &self.id }
    fn style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    
    fn render(&self) -> VNode {
        VNode::element("div")
            .with_style(self.get_style().read().unwrap().clone())
            .with_children(self.children.render_all())
            .with_key(self.id.clone())
    }
}

impl<'a> Stylable for Flex<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
