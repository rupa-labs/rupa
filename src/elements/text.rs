use crate::utils::{Style, StyleModifier, generate_id, Theme, Color, Attributes, Accessibility, TextAlign};
use crate::Component;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Text { pub id: String, pub content: String, pub style: Style, pub attributes: Attributes, pub accessibility: Accessibility }
impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), content: content.into(), style, attributes: Attributes::default(), accessibility: Accessibility::default() }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}
impl Component for Text {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.style.to_taffy()).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        let color = style.typography.color.clone().unwrap_or(Color::white(1.0)).to_rgba();
        renderer.draw_text(&self.content, layout.location.x, layout.location.y, style.typography.base_size(), color, style.typography.align.clone());
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _: f32) {}
    fn on_drag(&self, _: crate::utils::Vec2) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_text_creation() { let txt = Text::new("Artisan"); assert_eq!(txt.content, "Artisan"); }
}
