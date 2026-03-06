use crate::utils::{Style, generate_id, StyleModifier, Signal, Theme, Color, Accessibility, Attributes, TextAlign};
use crate::Component;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Label {
    pub id: String,
    pub text: String,
    pub style: Style,
    pub attributes: Attributes,
    pub accessibility: Accessibility,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), text: text.into(), style, attributes: Attributes::default(), accessibility: Accessibility::default() }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}

impl Component for Label {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.style.to_taffy()).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_text(&self.text, layout.location.x, layout.location.y, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _: f32) {}
    fn on_drag(&self, _: crate::utils::Vec2) {}
}

pub struct Checkbox {
    pub id: String,
    pub checked: Signal<bool>,
    pub style: Style,
    pub accessibility: Accessibility,
    pub attributes: Attributes,
}

impl Checkbox {
    pub fn new(checked: Signal<bool>) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), checked, style, accessibility: Accessibility { role: crate::utils::Role::Checkbox, ..Default::default() }, attributes: Attributes::default() }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}

impl Component for Checkbox {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let mut style = self.style.to_taffy();
        style.size = Size { width: length(20.0), height: length(20.0) };
        let node = taffy.new_leaf(style).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap();
        let color = if self.checked.get() { [0.39, 0.45, 1.0, 1.0] } else { [0.2, 0.2, 0.2, 1.0] };
        renderer.draw_rect(layout.location.x, layout.location.y, 20.0, 20.0, color, 4.0);
    }
    fn on_click(&self) { self.checked.update(|v| *v = !*v); }
    fn on_scroll(&self, _: f32) {}
    fn on_drag(&self, _: crate::utils::Vec2) {}
}

pub struct Radio { pub id: String, pub selected: Signal<bool>, pub style: Style }
impl Radio { pub fn new(selected: Signal<bool>) -> Self { Self { id: generate_id(), selected, style: Style::default() } } }
impl Component for Radio {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let mut s = self.style.to_taffy(); s.size = Size { width: length(20.0), height: length(20.0) };
        let node = taffy.new_leaf(s).unwrap(); if let Some(p) = parent { taffy.add_child(p, node).unwrap(); } node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap(); renderer.draw_rect(layout.location.x, layout.location.y, 20.0, 20.0, [0.3, 0.3, 0.3, 1.0], 10.0);
        if self.selected.get() { renderer.draw_rect(layout.location.x + 5.0, layout.location.y + 5.0, 10.0, 10.0, [0.39, 0.45, 1.0, 1.0], 5.0); }
    }
    fn on_click(&self) { self.selected.set(true); }
    fn on_scroll(&self, _: f32) {}
    fn on_drag(&self, _: crate::utils::Vec2) {}
}

pub struct Select { pub id: String, pub options: Vec<String>, pub selected_index: Signal<usize>, pub style: Style }
impl Select { pub fn new(options: Vec<String>, selected: Signal<usize>) -> Self { Self { id: generate_id(), options, selected_index: selected, style: Style::default() } } }
impl Component for Select {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.style.to_taffy()).unwrap(); if let Some(p) = parent { taffy.add_child(p, node).unwrap(); } node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap(); renderer.draw_rect(layout.location.x, layout.location.y, layout.size.width, layout.size.height, [0.15, 0.15, 0.15, 1.0], 4.0);
        let text = self.options.get(self.selected_index.get()).cloned().unwrap_or_default(); renderer.draw_text(&text, layout.location.x + 8.0, layout.location.y + 8.0, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _: f32) {}
    fn on_drag(&self, _: crate::utils::Vec2) {}
}

pub struct Switch { pub id: String, pub on: Signal<bool>, pub style: Style }
impl Switch { pub fn new(on: Signal<bool>) -> Self { Self { id: generate_id(), on, style: Style::default() } } }
impl Component for Switch {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let mut s = self.style.to_taffy(); s.size = Size { width: length(40.0), height: length(20.0) };
        let node = taffy.new_leaf(s).unwrap(); if let Some(p) = parent { taffy.add_child(p, node).unwrap(); } node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap(); let bg_color = if self.on.get() { [0.15, 0.68, 0.37, 1.0] } else { [0.3, 0.3, 0.3, 1.0] }; renderer.draw_rect(layout.location.x, layout.location.y, 40.0, 20.0, bg_color, 10.0);
        let knob_x = if self.on.get() { 22.0 } else { 2.0 }; renderer.draw_rect(layout.location.x + knob_x, layout.location.y + 2.0, 16.0, 16.0, [1.0, 1.0, 1.0, 1.0], 8.0);
    }
    fn on_click(&self) { self.on.update(|v| *v = !*v); }
    fn on_scroll(&self, _: f32) {}
    fn on_drag(&self, _: crate::utils::Vec2) {}
}

pub struct Input { pub id: String, pub value: Signal<String>, pub placeholder: String, pub style: Style, pub accessibility: Accessibility, pub attributes: Attributes }
impl Input {
    pub fn new(initial_value: impl Into<String>) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        style.background.color = Some(Color::Semantic("surface".into(), None)); style.padding = crate::utils::spacing::Spacing::all(8.0);
        Self { id: generate_id(), value: Signal::new(initial_value.into()), placeholder: String::new(), style, accessibility: Accessibility::default(), attributes: Attributes::default() }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self { self.placeholder = p.into(); self }
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.attributes.set(key, value); self }
}
impl Component for Input {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.style.to_taffy()).unwrap(); if let Some(p) = parent { taffy.add_child(p, node).unwrap(); } node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        if let Some(color) = style.background.color.clone() { renderer.draw_rect(layout.location.x, layout.location.y, layout.size.width, layout.size.height, color.to_rgba(), 4.0); }
        let text = self.value.get(); let display_text = if text.is_empty() { &self.placeholder } else { &text };
        renderer.draw_text(display_text, layout.location.x + 8.0, layout.location.y + 8.0, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _: f32) {}
    fn on_drag(&self, _: crate::utils::Vec2) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_checkbox_toggle() { let val = Signal::new(false); let cb = Checkbox::new(val.clone()); cb.on_click(); assert_eq!(val.get(), true); }
}
