use rupa_core::{Component, VNode, Id};
use rupa_vnode::Style;
use crate::style::modifiers::Stylable;
use crate::primitives::Flex;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// A raw horizontal flex container.
pub struct Row<'a> {
    pub id: String,
    pub inner: Flex<'a>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl<'a> Row<'a> {
    pub fn new() -> Self {
        let inner = Flex::new();
        inner.get_style().write().unwrap().flex.flex_direction = rupa_vnode::FlexDirection::Row;
        Self {
            id: Id::next().to_string(),
            inner,
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }

    pub fn child(mut self, child: impl Component + 'a) -> Self {
        self.inner = self.inner.child(child);
        self
    }
}

impl<'a> Component for Row<'a> {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn children(&self) -> Vec<&dyn Component> { self.inner.children.as_refs() }
    
    fn render(&self) -> VNode {
        VNode::element("div")
            .with_style(self.inner.get_style().read().unwrap().clone())
            .with_children(self.inner.children.render_all())
            .with_key(self.id.clone())
    }
}

impl<'a> Stylable for Row<'a> {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.inner.get_style() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.inner.get_style_mut() }
}
