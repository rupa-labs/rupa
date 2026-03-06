use crate::utils::{StyleModifier, FlexDirection, Scale, Vec2};
use crate::core::component::Component;
use crate::primitives::flex::Flex;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;

pub struct VStack<'a> { inner: Flex<'a> }

impl<'a> VStack<'a> {
    pub fn new() -> Self { 
        Self { inner: Flex::new().direction(FlexDirection::Col) } 
    }
    pub fn gap(mut self, val: f32) -> Self { self.inner = self.inner.gap(val); self }
    pub fn gap_scale(mut self, s: Scale) -> Self { self.inner = self.inner.gap(s.value(16.0)); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.inner = self.inner.child(child); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { self.inner = self.inner.style(modifier); self }
}

impl<'a> Component for VStack<'a> {
    fn id(&self) -> &str { self.inner.id() }
    fn children(&self) -> Vec<&dyn Component> { self.inner.children() }
    fn get_node(&self) -> Option<NodeId> { self.inner.get_node() }
    fn set_node(&self, node: NodeId) { self.inner.set_node(node); }
    fn is_dirty(&self) -> bool { self.inner.is_dirty() }
    fn mark_dirty(&self) { self.inner.mark_dirty(); }
    fn clear_dirty(&self) { self.inner.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId { 
        self.inner.layout(taffy, parent) 
    }
    
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) { 
        self.inner.paint(renderer, taffy, node, is_group_hovered, render_pass, global_pos) 
    }
    
    fn on_click(&self, event: &mut UIEvent) { self.inner.on_click(event) }
    fn on_scroll(&self, event: &mut UIEvent, d: f32) { self.inner.on_scroll(event, d) }
    fn on_drag(&self, event: &mut UIEvent, d: Vec2) { self.inner.on_drag(event, d) }
}

pub struct HStack<'a> { inner: Flex<'a> }

impl<'a> HStack<'a> {
    pub fn new() -> Self { 
        Self { inner: Flex::new().direction(FlexDirection::Row) } 
    }
    pub fn gap(mut self, val: f32) -> Self { self.inner = self.inner.gap(val); self }
    pub fn gap_scale(mut self, s: Scale) -> Self { self.inner = self.inner.gap(s.value(16.0)); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.inner = self.inner.child(child); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { self.inner = self.inner.style(modifier); self }
}

impl<'a> Component for HStack<'a> {
    fn id(&self) -> &str { self.inner.id() }
    fn children(&self) -> Vec<&dyn Component> { self.inner.children() }
    fn get_node(&self) -> Option<NodeId> { self.inner.get_node() }
    fn set_node(&self, node: NodeId) { self.inner.set_node(node); }
    fn is_dirty(&self) -> bool { self.inner.is_dirty() }
    fn mark_dirty(&self) { self.inner.mark_dirty(); }
    fn clear_dirty(&self) { self.inner.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId { 
        self.inner.layout(taffy, parent) 
    }
    
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) { 
        self.inner.paint(renderer, taffy, node, is_group_hovered, render_pass, global_pos) 
    }
    
    fn on_click(&self, event: &mut UIEvent) { self.inner.on_click(event) }
    fn on_scroll(&self, event: &mut UIEvent, d: f32) { self.inner.on_scroll(event, d) }
    fn on_drag(&self, event: &mut UIEvent, d: Vec2) { self.inner.on_drag(event, d) }
}
