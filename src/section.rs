use crate::utils::{Style, StyleModifier, generate_id, Theme, Vec2};
use crate::Component;
use crate::container::Children;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Section<'a> {
    pub id: String,
    pub name: String,
    pub style: Style,
    pub children: Children<'a>,
}

impl<'a> Section<'a> {
    pub fn new(name: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: generate_id(),
            name: name.into(),
            style,
            children: Children::new(),
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.children.add(child); self }
}

impl<'a> Component for Section<'a> {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        self.children.layout_all(taffy, node);
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        if let Some(color) = style.background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw);
        }
        self.children.paint_all(renderer, taffy, node, is_group_hovered || self.style.is_group, render_pass, global_pos);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, delta: f32) { self.children.list.iter().for_each(|c| c.on_scroll(delta)); }
    fn on_drag(&self, delta: Vec2) { self.children.list.iter().for_each(|c| c.on_drag(delta)); }
}
