use crate::utils::{Style, generate_id, StyleModifier, Display, FlexDirection, Attributes, Accessibility, Overflow, Vec2, Signal};
use crate::Component;
use crate::container::Children;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Flex<'a> {
    pub id: String,
    pub style: Style,
    pub attributes: Attributes,
    pub accessibility: Accessibility,
    pub children: Children<'a>,
    pub scroll_offset: Signal<Vec2>,
}

impl<'a> Flex<'a> {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.layout.display = Display::Flex;
        Self {
            id: generate_id(), style, attributes: Attributes::default(),
            accessibility: Accessibility::default(), children: Children::new(),
            scroll_offset: Signal::new(Vec2::zero()),
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn direction(mut self, dir: FlexDirection) -> Self { self.style.flex.flex_direction = dir; self }
    pub fn gap(mut self, val: f32) -> Self { self.style.flex.gap = Some(val); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.children.add(child); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}

impl<'a> Component for Flex<'a> {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        self.children.layout_all(taffy, node);
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        if let Some(color) = style.background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw);
        }
        let needs_clip = self.style.layout.overflow_x != Overflow::Visible || self.style.layout.overflow_y != Overflow::Visible;
        if needs_clip { renderer.push_clip(global_pos.x, global_pos.y, layout.size.width, layout.size.height, render_pass); }
        let old_offset = renderer.camera_offset;
        renderer.camera_offset += self.scroll_offset.get();
        self.children.paint_all(renderer, taffy, node, is_group_hovered || self.style.is_group, render_pass, global_pos, 0);
        renderer.camera_offset = old_offset;
        if needs_clip { renderer.pop_clip(render_pass); }
    }
    fn on_click(&self) {}
    fn on_scroll(&self, delta: f32) { if self.style.layout.overflow_y == Overflow::Scroll { self.scroll_offset.update(|o| o.y += delta); } }
    fn on_drag(&self, delta: Vec2) { if self.style.layout.overflow_x == Overflow::Scroll || self.style.layout.overflow_y == Overflow::Scroll { self.scroll_offset.update(|o| { o.x += delta.x; o.y += delta.y; }); } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::FlexDirection;
    #[test]
    fn test_flex_properties() { let flex = Flex::new().direction(FlexDirection::Row).gap(20.0); assert_eq!(flex.style.flex.flex_direction, FlexDirection::Row); assert_eq!(flex.style.flex.gap, Some(20.0)); }
}
