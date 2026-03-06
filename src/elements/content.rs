use crate::utils::{Style, generate_id, Theme, Color, Accessibility, Attributes, Signal, TextAlign, Vec2};
use crate::Component;
use crate::container::Children;
use crate::renderer::Renderer;
use taffy::prelude::*;

pub struct Card<'a> {
    pub id: String, pub style: Style, pub accessibility: Accessibility, pub attributes: Attributes,
    pub header: Option<Box<dyn Component + 'a>>, pub body: Children<'a>, pub footer: Option<Box<dyn Component + 'a>>,
}

impl<'a> Card<'a> {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        style.background.color = Some(Color::Semantic("surface".into(), None));
        style.padding = crate::utils::spacing::Spacing::all(16.0);
        style.rounding = crate::utils::border::Rounding::all(8.0);
        Self { id: generate_id(), style, accessibility: Accessibility::default(), attributes: Attributes::default(), header: None, body: Children::new(), footer: None }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn header(mut self, h: Box<dyn Component + 'a>) -> Self { self.header = Some(h); self }
    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self { self.body.add(c); self }
    pub fn footer(mut self, f: Box<dyn Component + 'a>) -> Self { self.footer = Some(f); self }
}

impl<'a> Component for Card<'a> {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        if let Some(ref h) = self.header { h.layout(taffy, Some(node)); }
        self.body.layout_all(taffy, node);
        if let Some(ref f) = self.footer { f.layout(taffy, Some(node)); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        if let Some(color) = style.background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw); }
        
        let children_nodes = taffy.children(node).unwrap();
        let mut idx = 0;
        if let Some(ref h) = self.header { 
            if let Some(n) = children_nodes.get(idx) { 
                let child_layout = taffy.layout(*n).unwrap();
                h.paint(renderer, taffy, *n, false, render_pass, global_pos + Vec2::new(child_layout.location.x, child_layout.location.y)); 
                idx += 1; 
            } 
        }
        self.body.paint_all(renderer, taffy, node, is_group_hovered, render_pass, global_pos, idx);

    }
    fn on_click(&self) {}
    fn on_scroll(&self, d: f32) { self.body.list.iter().for_each(|c| c.on_scroll(d)); }
    fn on_drag(&self, d: Vec2) { self.body.list.iter().for_each(|c| c.on_drag(d)); }
}

pub struct TableRow { pub cells: Vec<String> }
pub struct Table { pub id: String, pub headers: Vec<String>, pub rows: Vec<TableRow>, pub style: Style }
impl Table { pub fn new(headers: Vec<String>, rows: Vec<TableRow>) -> Self { Self { id: generate_id(), headers, rows, style: Style::default() } } }
impl Component for Table {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let mut y_offset = 0.0;
        for header in &self.headers { renderer.draw_text(header, global_pos.x, global_pos.y + y_offset, 14.0, [0.6, 0.6, 0.6, 1.0], TextAlign::Left); }
        y_offset += 30.0;
        for row in &self.rows {
            let mut x_offset = 0.0;
            for cell in &row.cells { renderer.draw_text(cell, global_pos.x + x_offset, global_pos.y + y_offset, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left); x_offset += 100.0; }
            y_offset += 25.0;
        }
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _d: f32) {}
    fn on_drag(&self, _d: Vec2) {}
}

pub struct Accordion<'a> { pub id: String, pub title: String, pub is_expanded: Signal<bool>, pub children: Children<'a> }
impl<'a> Accordion<'a> {
    pub fn new(title: impl Into<String>, expanded: Signal<bool>) -> Self { Self { id: generate_id(), title: title.into(), is_expanded: expanded, children: Children::new() } }
    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self { self.children.add(c); self }
}
impl<'a> Component for Accordion<'a> {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(taffy::style::Style::default(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        if self.is_expanded.get() { self.children.layout_all(taffy, node); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        renderer.draw_text(&self.title, global_pos.x, global_pos.y, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
        if self.is_expanded.get() { self.children.paint_all(renderer, taffy, node, is_group_hovered, render_pass, global_pos, 0); }
    }
    fn on_click(&self) { self.is_expanded.update(|v| *v = !*v); }
    fn on_scroll(&self, d: f32) { if self.is_expanded.get() { self.children.list.iter().for_each(|c| c.on_scroll(d)); } }
    fn on_drag(&self, d: Vec2) { if self.is_expanded.get() { self.children.list.iter().for_each(|c| c.on_drag(d)); } }
}
