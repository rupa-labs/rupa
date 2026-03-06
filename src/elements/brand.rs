use crate::utils::{Style, generate_id, StyleModifier, Accessibility, Attributes, Color, TextAlign, Vec2};
use crate::Component;
use crate::renderer::{Renderer, Texture};
use taffy::prelude::*;

pub struct Brand {
    pub id: String,
    pub name: String,
    pub logo_path: Option<String>,
    pub texture: Option<Texture>,
    pub style: Style,
    pub accessibility: Accessibility,
    pub attributes: Attributes,
}

impl Brand {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: generate_id(), name: name.into(), logo_path: None, texture: None, style: Style::default(),
            accessibility: Accessibility { role: crate::utils::Role::Navigation, ..Default::default() },
            attributes: Attributes::default(),
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
    pub fn logo(mut self, path: impl Into<String>) -> Self { self.logo_path = Some(path.into()); self }
}

impl Component for Brand {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.style.to_taffy()).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let _layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        let color = style.typography.color.clone().unwrap_or(Color::white(1.0)).to_rgba();
        renderer.draw_text(&self.name, global_pos.x + 40.0, global_pos.y + 10.0, 18.0, color, TextAlign::Left);
        renderer.draw_rect(global_pos.x, global_pos.y, 32.0, 32.0, [0.39, 0.45, 1.0, 1.0], style.rounding.nw);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: Vec2) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_brand_creation() { let brand = Brand::new("Rupa Art"); assert_eq!(brand.name, "Rupa Art"); }
}
