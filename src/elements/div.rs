use crate::utils::{Style, StyleModifier, generate_id, Attributes, Theme, Accessibility, Overflow, Vec2, Signal};
use crate::Component;
use crate::container::Children;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Div {
    pub id: String,
    pub style: Style,
    pub accessibility: Accessibility,
    pub attributes: Attributes,
    pub children: Children,
    pub scroll_offset: Signal<Vec2>,
}

impl Div {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: generate_id(), style, accessibility: Accessibility::default(),
            attributes: Attributes::new(), children: Children::new(),
            scroll_offset: Signal::new(Vec2::zero()),
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
}

impl Component for Div {
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
        if let Some(color) = style.background.color.clone() {
            renderer.draw_rect(layout.location.x, layout.location.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw);
        }
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
