use crate::utils::{Style, generate_id, StyleModifier, Vec2, Attributes, Accessibility, Theme};
use crate::Component;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Path { pub points: Vec<Vec2>, pub color: [f32; 4], pub thickness: f32 }

pub struct Icon {
    pub id: String,
    pub name: String,
    pub size: f32,
    pub color: [f32; 4],
}

impl Icon {
    pub fn new(name: impl Into<String>) -> Self {
        Self { id: generate_id(), name: name.into(), size: 24.0, color: [1.0, 1.0, 1.0, 1.0] }
    }
    pub fn size(mut self, s: f32) -> Self { self.size = s; self }
    pub fn color(mut self, c: [f32; 4]) -> Self { self.color = c; self }
}

impl Component for Icon {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let mut s = taffy::style::Style::default();
        s.size = Size { width: length(self.size), height: length(self.size) };
        let node = taffy.new_leaf(s).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let _layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, self.size, self.size, self.color, 2.0);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: Vec2) {}
}

pub struct SvgCanvas {
    pub id: String, pub paths: Vec<Path>, pub style: Style, pub attributes: Attributes, pub accessibility: Accessibility,
}

impl SvgCanvas {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), paths: Vec::new(), style, attributes: Attributes::default(), accessibility: Accessibility::default() }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
    pub fn add_path(&mut self, path: Path) { self.paths.push(path); }
}

impl Component for SvgCanvas {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.style.to_taffy()).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        if let Some(color) = style.background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), 0.0);
        }
        for path in &self.paths {
            for point in &path.points {
                renderer.draw_rect(global_pos.x + point.x, global_pos.y + point.y, path.thickness, path.thickness, path.color, 0.0);
            }
        }
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: Vec2) {}
}
