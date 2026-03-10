use rupa_core::{Component, VNode, Vec2, ViewCore, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::Style;
use crate::style::modifiers::base::Stylable;
use crate::primitives::flex::Flex;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

pub struct VStack<'a> {
    pub inner: Flex<'a>,
}

impl<'a> VStack<'a> {
    pub fn new() -> Self {
        let inner = Flex::new();
        inner.view.style().flex.flex_direction = rupa_vnode::FlexDirection::Col;
        Self { inner }
    }

    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self {
        self.inner = self.inner.child(child);
        self
    }

    pub fn gap(self, val: f32) -> Self {
        self.inner.view.style().flex.gap = Some(val);
        self
    }
}

impl<'a> Stylable for VStack<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.inner.view.style() }
}

impl<'a> Component for VStack<'a> {
    fn id(&self) -> &str { self.inner.id() }
    fn children(&self) -> Vec<&dyn Component> { self.inner.children() }
    fn view_core(&self) -> Arc<ViewCore> { self.inner.view_core() }
    fn render(&self) -> VNode { self.inner.render() }

    fn get_node(&self) -> Option<SceneNode> { self.inner.get_node() }
    fn set_node(&self, node: SceneNode) { self.inner.set_node(node); }
    fn is_dirty(&self) -> bool { self.inner.is_dirty() }
    fn mark_dirty(&self) { self.inner.mark_dirty(); }
    fn clear_dirty(&self) { self.inner.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        self.inner.layout(taffy, measurer, parent)
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        self.inner.paint(renderer, taffy, node, is_group_hovered, global_pos);
    }
}
