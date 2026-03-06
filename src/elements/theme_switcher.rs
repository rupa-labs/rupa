use crate::utils::{Style, StyleModifier, generate_id, Theme, ColorMode, Variant, Accessibility, Attributes, Vec2};

use crate::Component;
use crate::elements::Button;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct ThemeSwitcher {
    pub id: String,
    pub style: Style,
    pub accessibility: Accessibility,
    pub attributes: Attributes,
}

impl ThemeSwitcher {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: generate_id(), style, accessibility: Accessibility::default(), attributes: Attributes::default(),
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
    
    pub fn cycle_mode() {
        Theme::update(|t| {
            t.mode = match t.mode { ColorMode::Dark => ColorMode::Light, ColorMode::Light => ColorMode::System, ColorMode::System => ColorMode::Dark };
        });
    }

    fn get_button(&self) -> Button<'_> {
        let label = match Theme::current().mode { ColorMode::Dark => "Dark", ColorMode::Light => "Light", ColorMode::System => "System" };
        Button::new(label).variant(Variant::Secondary).style(self.style.clone()).on_click(|| Self::cycle_mode())
    }
}

impl Component for ThemeSwitcher {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId { self.get_button().layout(taffy, parent) }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        self.get_button().paint(renderer, taffy, node, is_group_hovered, render_pass, global_pos);
    }
    fn on_click(&self) { self.get_button().trigger(); }
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: crate::utils::Vec2) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{Theme, ColorMode};
    #[test]
    fn test_theme_cycling() { Theme::update(|t| t.mode = ColorMode::Dark); assert_eq!(Theme::current().mode, ColorMode::Dark); ThemeSwitcher::cycle_mode(); assert_eq!(Theme::current().mode, ColorMode::Light); }
}
