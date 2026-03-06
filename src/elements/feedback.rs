use crate::utils::{Style, generate_id, StyleModifier, Theme, Color, Variant, Accessibility, Attributes, Signal, TextAlign, Vec2};
use crate::Component;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Progress {
    pub id: String,
    pub value: Signal<f32>,
    pub style: Style,
}

impl Progress {
    pub fn new(value: Signal<f32>) -> Self {
        Self { id: generate_id(), value, style: Style::default() }
    }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}

impl Component for Progress {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let mut s = self.style.to_taffy();
        if s.size.height == Dimension::Auto { s.size.height = length(10.0); }
        let node = taffy.new_leaf(s).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.2, 0.2, 0.2, 1.0], 5.0);
        let fill_w = layout.size.width * self.value.get().clamp(0.0, 1.0);
        let color = self.style.background.color.clone().unwrap_or(Color::Semantic("primary".into(), None)).to_rgba();
        renderer.draw_rect(global_pos.x, global_pos.y, fill_w, layout.size.height, color, 5.0);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: Vec2) {}
}

pub struct Skeleton {
    pub id: String,
    pub style: Style,
}

impl Skeleton {
    pub fn new() -> Self {
        Self { id: generate_id(), style: Style::default() }
    }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}

impl Component for Skeleton {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.style.to_taffy()).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.15, 0.15, 0.15, 1.0], 4.0);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: Vec2) {}
}

pub struct Badge {
    pub id: String,
    pub label: String,
    pub variant: Variant,
    pub style: Style,
    pub accessibility: Accessibility,
    pub attributes: Attributes,
}

impl Badge {
    pub fn new(label: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        style.padding = crate::utils::spacing::Spacing::all(4.0);
        style.rounding = crate::utils::border::Rounding::all(99.0);
        Self { id: generate_id(), label: label.into(), variant: Variant::Primary, style, accessibility: Accessibility::default(), attributes: Attributes::default() }
    }
}

impl Component for Badge {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.style.to_taffy()).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        let color = style.background.color.clone().unwrap_or(Color::Semantic("primary".into(), None));
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), 10.0);
        renderer.draw_text(&self.label, global_pos.x + 6.0, global_pos.y + 2.0, 12.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: Vec2) {}
}

pub struct Spinner {
    pub id: String,
    pub style: Style,
}

impl Spinner {
    pub fn new() -> Self {
        Self { id: generate_id(), style: Style::default() }
    }
}

impl Component for Spinner {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let mut s = taffy::style::Style::default();
        s.size = Size { width: length(24.0), height: length(24.0) };
        let node = taffy.new_leaf(s).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let _layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, 24.0, 24.0, [0.5, 0.5, 0.5, 1.0], 12.0);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: Vec2) {}
}

pub struct Alert {
    pub id: String,
    pub message: String,
    pub variant: Variant,
    pub style: Style,
    pub accessibility: Accessibility,
    pub attributes: Attributes,
}

impl Alert {
    pub fn new(message: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        style.padding = crate::utils::spacing::Spacing::all(16.0);
        Self { id: generate_id(), message: message.into(), variant: Variant::Info, style, accessibility: Accessibility { role: crate::utils::Role::Alert, ..Default::default() }, attributes: Attributes::default() }
    }
}

impl Component for Alert {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.style.to_taffy()).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        let color = style.background.color.clone().unwrap_or(Color::Semantic("surface".into(), None));
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), 4.0);
        renderer.draw_text(&self.message, global_pos.x + 16.0, global_pos.y + 12.0, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: Vec2) {}
}
