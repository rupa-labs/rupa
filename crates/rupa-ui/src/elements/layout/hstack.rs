use rupa_core::vnode::VNode; use rupa_core::component::Component;
use rupa_core::{FlexDirection, Vec2, Style};
use rupa_core::component::Component;
use crate::primitives::flex::Flex;
use rupa_core::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::base::Stylable;
use rupa_core::scene::SceneNode;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

/// A specialized Flex container that stacks children horizontally.
pub struct HStack<'a> {
    pub inner: Flex<'a>,
}

impl<'a> HStack<'a> {
    pub fn new() -> Self {
        let flex = Flex::new();
        flex.view.core.get_style_mut().flex.flex_direction = FlexDirection::Row;
        Self { inner: flex }
    }

    pub fn gap(self, val: f32) -> Self {
        self.inner.view.core.get_style_mut().flex.gap = Some(val);
        self
    }

    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self {
        self.inner = self.inner.child(c);
        self
    }
}

impl<'a> Stylable for HStack<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.inner.view.core.get_style_mut() }
}

impl<'a> Component for HStack<'a> {
    fn render(&self) -> VNode { VNode::Empty }
    fn id(&self) -> &str { self.inner.id() }
    fn children(&self) -> Vec<&dyn Component> { self.inner.children() }
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
