use crate::utils::{Style, generate_id, StyleModifier, Signal, Vec2};
use crate::Component;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Viewport {
    pub id: String, pub style: Style, pub content: Box<dyn Component>, pub offset: Signal<Vec2>, pub zoom: Signal<f32>, pub zoomable: bool, pub pannable: bool,
}

impl Viewport {
    pub fn new(content: Box<dyn Component>) -> Self {
        Self { id: generate_id(), style: Style::default(), content, offset: Signal::new(Vec2::zero()), zoom: Signal::new(1.0), zoomable: true, pannable: true }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
}

impl Component for Viewport {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        self.content.layout(taffy, Some(node));
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>) {
        let old_offset = renderer.camera_offset; let old_zoom = renderer.camera_zoom;
        renderer.camera_offset = self.offset.get(); renderer.camera_zoom = self.zoom.get();
        let children = taffy.children(node).unwrap();
        if let Some(content_node) = children.get(0) { self.content.paint(renderer, taffy, *content_node, is_group_hovered, render_pass); }
        renderer.camera_offset = old_offset; renderer.camera_zoom = old_zoom;
    }
    fn on_click(&self) { self.content.on_click(); }
    fn on_scroll(&self, delta: f32) { if self.zoomable { self.zoom.update(|z| { *z += delta * 0.001; *z = z.clamp(0.1, 10.0); }); } }
    fn on_drag(&self, delta: Vec2) { if self.pannable { self.offset.update(|o| { o.x += delta.x / self.zoom.get(); o.y += delta.y / self.zoom.get(); }); } }
}
