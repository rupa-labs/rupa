use crate::Component;
use crate::utils::{Style, generate_id, StyleModifier, Accessibility, Attributes, Overflow, Vec2, Signal, Theme};
use crate::container::Children;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Container<'a> {
    pub id: String, pub style: Style, pub accessibility: Accessibility, pub attributes: Attributes, pub children: Children<'a>, pub scroll_offset: Signal<Vec2>,
}

impl<'a> Container<'a> {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), style, accessibility: Accessibility::default(), attributes: Attributes::default(), children: Children::new(), scroll_offset: Signal::new(Vec2::zero()) }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.children.add(child); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}

impl<'a> Component for Container<'a> {
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
            // Use global_pos instead of local layout.location
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw);
        }

        let needs_clip = self.style.layout.overflow_x != Overflow::Visible || self.style.layout.overflow_y != Overflow::Visible;
        if needs_clip { renderer.push_clip(global_pos.x, global_pos.y, layout.size.width, layout.size.height, render_pass); }
        
        // Pass the same global_pos to children (paint_all will handle the offsets)
        self.children.paint_all(renderer, taffy, node, is_group_hovered || self.style.is_group, render_pass, global_pos + self.scroll_offset.get(), 0);
        
        if needs_clip { renderer.pop_clip(render_pass); }
    }
    fn on_click(&self) {}
    fn on_scroll(&self, delta: f32) { if self.style.layout.overflow_y == Overflow::Scroll { self.scroll_offset.update(|o| o.y += delta); } }
    fn on_drag(&self, delta: Vec2) { if self.style.layout.overflow_x == Overflow::Scroll || self.style.layout.overflow_y == Overflow::Scroll { self.scroll_offset.update(|o| { o.x += delta.x; o.y += delta.y; }); } }
}

pub struct Row<'a> { pub id: String, pub style: Style, pub accessibility: Accessibility, pub attributes: Attributes, pub children: Children<'a> }
impl<'a> Row<'a> {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        style.layout.display = crate::utils::Display::Flex; style.flex.flex_direction = crate::utils::FlexDirection::Row;
        Self { id: generate_id(), style, accessibility: Accessibility::default(), attributes: Attributes::default(), children: Children::new() }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.children.add(child); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}
impl<'a> Component for Row<'a> {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        self.children.layout_all(taffy, node);
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        self.children.paint_all(renderer, taffy, node, is_group_hovered || self.style.is_group, render_pass, global_pos, 0);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, delta: f32) { self.children.list.iter().for_each(|c| c.on_scroll(delta)); }
    fn on_drag(&self, delta: Vec2) { self.children.list.iter().for_each(|c| c.on_drag(delta)); }
}

pub struct Col<'a> { pub id: String, pub style: Style, pub accessibility: Accessibility, pub attributes: Attributes, pub children: Children<'a> }
impl<'a> Col<'a> {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        style.layout.display = crate::utils::Display::Flex; style.flex.flex_direction = crate::utils::FlexDirection::Col;
        Self { id: generate_id(), style, accessibility: Accessibility::default(), attributes: Attributes::default(), children: Children::new() }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.children.add(child); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}
impl<'a> Component for Col<'a> {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        self.children.layout_all(taffy, node);
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        self.children.paint_all(renderer, taffy, node, is_group_hovered || self.style.is_group, render_pass, global_pos, 0);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, delta: f32) { self.children.list.iter().for_each(|c| c.on_scroll(delta)); }
    fn on_drag(&self, delta: Vec2) { self.children.list.iter().for_each(|c| c.on_drag(delta)); }
}
