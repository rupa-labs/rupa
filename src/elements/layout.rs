use crate::Component;
use crate::utils::{Style, generate_id, StyleModifier, Accessibility, Attributes, Overflow, Vec2, Signal, Theme};
use crate::container::Children;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Container {
    pub id: String,
    pub style: Style,
    pub accessibility: Accessibility,
    pub attributes: Attributes,
    pub children: Children,
    pub scroll_offset: Signal<Vec2>,
}

impl Container {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), style, accessibility: Accessibility::default(), attributes: Attributes::default(), children: Children::new(), scroll_offset: Signal::new(Vec2::zero()) }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.attributes.set(key, value); self }
}

impl Component for Container {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        self.children.layout_all(taffy, node);
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        if let Some(color) = style.background.color.clone() { renderer.draw_rect(layout.location.x, layout.location.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw); }
        let needs_clip = self.style.layout.overflow_x != Overflow::Visible || self.style.layout.overflow_y != Overflow::Visible;
        if needs_clip { renderer.push_clip(layout.location.x, layout.location.y, layout.size.width, layout.size.height, render_pass); }
        let old_offset = renderer.camera_offset;
        renderer.camera_offset += self.scroll_offset.get();
        self.children.paint_all(renderer, taffy, node, is_group_hovered || self.style.is_group, render_pass);
        renderer.camera_offset = old_offset;
        if needs_clip { renderer.pop_clip(render_pass); }
    }
    fn on_click(&self) {}
    fn on_scroll(&self, delta: f32) { if self.style.layout.overflow_y == Overflow::Scroll { self.scroll_offset.update(|o| o.y += delta); } }
    fn on_drag(&self, delta: Vec2) { if self.style.layout.overflow_x == Overflow::Scroll || self.style.layout.overflow_y == Overflow::Scroll { self.scroll_offset.update(|o| { o.x += delta.x; o.y += delta.y; }); } }
}

pub struct Row { pub id: String, pub style: Style, pub accessibility: Accessibility, pub attributes: Attributes, pub children: Children }
impl Row {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        style.layout.display = crate::utils::Display::Flex; style.flex.flex_direction = crate::utils::FlexDirection::Row;
        Self { id: generate_id(), style, accessibility: Accessibility::default(), attributes: Attributes::default(), children: Children::new() }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}
impl Component for Row {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        self.children.layout_all(taffy, node);
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        if let Some(color) = style.background.color.clone() { renderer.draw_rect(layout.location.x, layout.location.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw); }
        self.children.paint_all(renderer, taffy, node, is_group_hovered || self.style.is_group, render_pass);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, delta: f32) { self.children.list.iter().for_each(|c| c.on_scroll(delta)); }
    fn on_drag(&self, delta: Vec2) { self.children.list.iter().for_each(|c| c.on_drag(delta)); }
}

pub struct Col { pub id: String, pub style: Style, pub accessibility: Accessibility, pub attributes: Attributes, pub children: Children }
impl Col {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        style.layout.display = crate::utils::Display::Flex; style.flex.flex_direction = crate::utils::FlexDirection::Col;
        Self { id: generate_id(), style, accessibility: Accessibility::default(), attributes: Attributes::default(), children: Children::new() }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}
impl Component for Col {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        self.children.layout_all(taffy, node);
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        if let Some(color) = style.background.color.clone() { renderer.draw_rect(layout.location.x, layout.location.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw); }
        self.children.paint_all(renderer, taffy, node, is_group_hovered || self.style.is_group, render_pass);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, delta: f32) { self.children.list.iter().for_each(|c| c.on_scroll(delta)); }
    fn on_drag(&self, delta: Vec2) { self.children.list.iter().for_each(|c| c.on_drag(delta)); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::Text;
    #[test]
    fn test_container_creation() { let container = Container::new().child(Box::new(Text::new("Child"))); assert_eq!(container.children.list.len(), 1); }
    #[test]
    fn test_container_attr_a11y() { let container = Container::new().id("test-id").attr("data-test", "val"); assert_eq!(container.id, "test-id"); assert_eq!(container.attributes.get("data-test").unwrap(), "val"); }
}
