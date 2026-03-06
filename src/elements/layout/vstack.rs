use crate::support::{FlexDirection, Vec2, Style};
use crate::core::component::Component;
use crate::core::ViewCore;
use crate::primitives::flex::Flex;
use crate::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::base::Stylable;
use crate::platform::dispatcher::UIEvent;
use crate::scene::SceneNode;
use taffy::prelude::*;
use std::cell::RefMut;

/// A specialized Flex container that stacks children vertically.
pub struct VStack<'a> {
    pub inner: Flex<'a>,
}

impl<'a> VStack<'a> {
    pub fn new() -> Self {
        let mut flex = Flex::new();
        flex.view.core.get_style_mut().flex.flex_direction = FlexDirection::Col;
        Self { inner: flex }
    }

    pub fn gap(mut self, val: f32) -> Self {
        self.inner.view.core.get_style_mut().flex.gap = Some(val);
        self
    }

    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self {
        self.inner = self.inner.child(c);
        self
    }
}

impl<'a> Stylable for VStack<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.inner.view.core.get_style_mut() }
}

impl<'a> Component for VStack<'a> {
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

    fn on_click(&self, event: &mut UIEvent) { self.inner.on_click(event); }
    fn on_scroll(&self, event: &mut UIEvent, d: f32) { self.inner.on_scroll(event, d); }
    fn on_drag(&self, event: &mut UIEvent, d: Vec2) { self.inner.on_drag(event, d); }
}
