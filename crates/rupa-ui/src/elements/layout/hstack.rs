use rupa_core::{Component, VNode, ViewCore, Id};
use rupa_vnode::Style;
use crate::style::modifiers::Stylable;
use crate::primitives::Flex;
use std::sync::{RwLockWriteGuard, Arc};

/// A horizontal stacking container.
pub struct HStack<'a> {
    pub id: String,
    pub inner: Flex<'a>,
}

impl<'a> HStack<'a> {
    pub fn new() -> Self {
        let inner = Flex::new();
        inner.view.style().flex.flex_direction = rupa_vnode::FlexDirection::Row;
        Self {
            id: Id::next().to_string(),
            inner,
        }
    }

    pub fn child(mut self, child: impl Component + 'a) -> Self {
        self.inner = self.inner.child(Box::new(child));
        self
    }
}

impl<'a> Component for HStack<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.inner.children() }
    fn view_core(&self) -> Arc<ViewCore> { self.inner.view_core() }
    fn render(&self) -> VNode { self.inner.render() }
}

impl<'a> Stylable for HStack<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.inner.get_style_mut() }
}
