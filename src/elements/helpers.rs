use crate::utils::{StyleModifier, FlexDirection, Scale};
use crate::Component;
use crate::elements::Flex;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct VStack<'a> { inner: Flex<'a> }
impl<'a> VStack<'a> {
    pub fn new() -> Self { Self { inner: Flex::new().direction(FlexDirection::Col) } }
    pub fn gap(mut self, val: f32) -> Self { self.inner = self.inner.gap(val); self }
    pub fn gap_scale(mut self, s: Scale) -> Self { self.inner = self.inner.gap(s.value(16.0)); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.inner = self.inner.child(child); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { self.inner = self.inner.style(modifier); self }
}
impl<'a> Component for VStack<'a> {
    fn id(&self) -> &str { self.inner.id.as_str() }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId { self.inner.layout(taffy, parent) }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: crate::utils::Vec2) { self.inner.paint(renderer, taffy, node, is_group_hovered, _render_pass, global_pos) }
    fn on_click(&self) { self.inner.on_click() }
    fn on_scroll(&self, d: f32) { self.inner.on_scroll(d) }
    fn on_drag(&self, d: crate::utils::Vec2) { self.inner.on_drag(d) }
}

pub struct HStack<'a> { inner: Flex<'a> }
impl<'a> HStack<'a> {
    pub fn new() -> Self { Self { inner: Flex::new().direction(FlexDirection::Row) } }
    pub fn gap(mut self, val: f32) -> Self { self.inner = self.inner.gap(val); self }
    pub fn gap_scale(mut self, s: Scale) -> Self { self.inner = self.inner.gap(s.value(16.0)); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.inner = self.inner.child(child); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { self.inner = self.inner.style(modifier); self }
}
impl<'a> Component for HStack<'a> {
    fn id(&self) -> &str { self.inner.id.as_str() }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId { self.inner.layout(taffy, parent) }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: crate::utils::Vec2) { self.inner.paint(renderer, taffy, node, is_group_hovered, _render_pass, global_pos) }
    fn on_click(&self) { self.inner.on_click() }
    fn on_scroll(&self, d: f32) { self.inner.on_scroll(d) }
    fn on_drag(&self, d: crate::utils::Vec2) { self.inner.on_drag(d) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::FlexDirection;
    #[test]
    fn test_stack_directions() { assert_eq!(VStack::new().inner.style.flex.flex_direction, FlexDirection::Col); assert_eq!(HStack::new().inner.style.flex.flex_direction, FlexDirection::Row); }
}
