use crate::utils::{Style, generate_id, Signal, Theme, Color, Accessibility, Attributes, TextAlign, Vec2};
use crate::Component;
use crate::container::Children;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Modal<'a> {
    pub id: String, pub is_open: Signal<bool>, pub style: Style, pub accessibility: Accessibility, pub attributes: Attributes, pub children: Children<'a>,
    pub header: Option<Box<dyn Component + 'a>>, pub footer: Option<Box<dyn Component + 'a>>,
}

impl<'a> Modal<'a> {
    pub fn new(is_open: Signal<bool>) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        style.background.color = Some(Color::Semantic("surface".into(), None));
        style.sizing.width = Some(400.0); style.padding = crate::utils::spacing::Spacing::all(24.0);
        Self { id: generate_id(), is_open, style, accessibility: Accessibility { role: crate::utils::Role::Alert, ..Default::default() }, attributes: Attributes::default(), children: Children::new(), header: None, footer: None }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
}

impl<'a> Component for Modal<'a> {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        if !self.is_open.get() { return taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::None, ..Default::default() }).unwrap(); }
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        if let Some(ref h) = self.header { h.layout(taffy, Some(node)); }
        self.children.layout_all(taffy, node);
        if let Some(ref f) = self.footer { f.layout(taffy, Some(node)); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        if !self.is_open.get() { return; }
        let layout = taffy.layout(node).unwrap();
        if let Some(color) = self.style.background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), 12.0); }
        self.children.paint_all(renderer, taffy, node, is_group_hovered, render_pass, global_pos, 0);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, d: f32) { if self.is_open.get() { self.children.list.iter().for_each(|c| c.on_scroll(d)); } }
    fn on_drag(&self, d: crate::utils::Vec2) { if self.is_open.get() { self.children.list.iter().for_each(|c| c.on_drag(d)); } }
}

pub struct Tooltip { pub id: String, pub text: String, pub style: Style }
impl Tooltip { pub fn new(text: impl Into<String>) -> Self { Self { id: generate_id(), text: text.into(), style: Style::default() } } }
impl Component for Tooltip {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.style.to_taffy()).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.1, 0.1, 0.1, 0.9], 4.0);
        renderer.draw_text(&self.text, global_pos.x + 4.0, global_pos.y + 2.0, 12.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _: f32) {}
    fn on_drag(&self, _: Vec2) {}
}
