use crate::support::{Style, generate_id, Theme, Color, Accessibility, Attributes, Signal, TextAlign, Vec2, Display, FlexDirection, JustifyContent};
use crate::core::component::Component;
use crate::core::ViewCore;
use crate::elements::text::Text;
use crate::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use crate::scene::SceneNode;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- CARD ---
pub struct CardLogic<'a> { pub header: Option<Box<dyn Component + 'a>>, pub body: crate::elements::layout::container::Children<'a>, pub footer: Option<Box<dyn Component + 'a>>, pub accessibility: Accessibility }
pub struct CardView { pub core: ViewCore }
pub struct Card<'a> { pub id: String, pub logic: CardLogic<'a>, pub view: CardView }
impl<'a> Card<'a> {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        style.background.color = Some(Color::Semantic("surface".into(), None));
        style.padding = crate::support::Spacing::all(16.0);
        style.rounding = crate::support::Rounding::all(8.0);
        Self { id: generate_id(), logic: CardLogic { header: None, body: crate::elements::layout::container::Children::new(), footer: None, accessibility: Accessibility::default() }, view: CardView { core: ViewCore::new(style) } }
    }
    pub fn header(mut self, h: Box<dyn Component + 'a>) -> Self { self.logic.header = Some(h); self.view.core.mark_dirty(); self }
    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self { self.logic.body.add(c); self.view.core.mark_dirty(); self }
    pub fn footer(mut self, f: Box<dyn Component + 'a>) -> Self { self.logic.footer = Some(f); self.view.core.mark_dirty(); self }
}
impl<'a> Stylable for Card<'a> { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl<'a> Component for Card<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> {
        let mut c = Vec::new(); if let Some(ref h) = self.logic.header { c.push(h.as_ref()); }
        c.extend(self.logic.body.get_all()); if let Some(ref f) = self.logic.footer { c.push(f.as_ref()); }
        c
    }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(self.view.core.get_style_mut().to_taffy(), &[]).unwrap();
            self.view.core.set_node(SceneNode::from(new_node)); new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        let mut child_nodes = Vec::new();
        if let Some(ref h) = self.logic.header { child_nodes.push(h.layout(taffy, measurer, Some(node))); }
        child_nodes.extend(self.logic.body.layout_all(taffy, measurer, node));
        if let Some(ref f) = self.logic.footer { child_nodes.push(f.layout(taffy, measurer, Some(node))); }
        taffy.set_children(node, &child_nodes).unwrap();
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap(); let style = self.view.core.style.borrow();
        if let Some(color) = style.background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw); }
        let children_nodes = taffy.children(node).unwrap(); let mut idx = 0;
        if let Some(ref h) = self.logic.header { if let Some(n) = children_nodes.get(idx) { let child_layout = taffy.layout(*n).unwrap(); h.paint(renderer, taffy, *n, false, global_pos + Vec2::new(child_layout.location.x, child_layout.location.y)); idx += 1; } }
        self.logic.body.paint_all(renderer, taffy, node, is_group_hovered, global_pos, idx);
        if let Some(ref f) = self.logic.footer { if let Some(n) = children_nodes.last() { let child_layout = taffy.layout(*n).unwrap(); f.paint(renderer, taffy, *n, false, global_pos + Vec2::new(child_layout.location.x, child_layout.location.y)); } }
    }
    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, ev: &mut UIEvent, d: f32) { self.logic.body.list.iter().for_each(|c| c.on_scroll(ev, d)); }
    fn on_drag(&self, ev: &mut UIEvent, d: Vec2) { self.logic.body.list.iter().for_each(|c| c.on_drag(ev, d)); }
}

// --- TABLE ---
pub struct TableRow { pub cells: Vec<String> }
pub struct TableLogic<'a> { pub headers: Vec<String>, pub rows: Vec<TableRow>, pub children: crate::elements::layout::container::Children<'a> }
pub struct TableView { pub core: ViewCore }
pub struct Table<'a> { pub id: String, pub logic: TableLogic<'a>, pub view: TableView }
impl<'a> Table<'a> {
    pub fn new(headers: Vec<String>, rows: Vec<TableRow>) -> Self {
        let mut style = Style::default(); style.layout.display = Display::Flex; style.flex.flex_direction = FlexDirection::Col; style.flex.gap = Some(8.0);
        Self { id: generate_id(), logic: TableLogic { headers, rows, children: crate::elements::layout::container::Children::new() }, view: TableView { core: ViewCore::new(style) } }
    }
}
impl<'a> Stylable for Table<'a> { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl<'a> Component for Table<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.get_all() }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(self.view.core.get_style_mut().to_taffy(), &[]).unwrap();
            self.view.core.set_node(SceneNode::from(new_node)); new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.logic.children.layout_all(taffy, measurer, node);
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        if let Some(color) = self.view.core.style.borrow().background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), 0.0); }
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered, global_pos, 0);
    }
    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, ev: &mut UIEvent, d: f32) { self.logic.children.list.iter().for_each(|c| c.on_scroll(ev, d)); }
    fn on_drag(&self, ev: &mut UIEvent, d: Vec2) { self.logic.children.list.iter().for_each(|c| c.on_drag(ev, d)); }
}

// --- ACCORDION ---
pub struct AccordionLogic<'a> { pub title: String, pub is_expanded: Signal<bool>, pub children: crate::elements::layout::container::Children<'a> }
pub struct AccordionView { pub core: ViewCore }
pub struct Accordion<'a> { pub id: String, pub logic: AccordionLogic<'a>, pub view: AccordionView }
impl<'a> Accordion<'a> {
    pub fn new(title: impl Into<String>, expanded: Signal<bool>) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        style.layout.display = Display::Flex; style.flex.flex_direction = FlexDirection::Col;
        style.background.color = Some(Color::Semantic("surface".into(), None));
        style.rounding = crate::support::Rounding::all(4.0);
        Self { id: generate_id(), logic: AccordionLogic { title: title.into(), is_expanded: expanded, children: crate::elements::layout::container::Children::new() }, view: AccordionView { core: ViewCore::new(style) } }
    }
    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self { self.logic.children.add(c); self.view.core.mark_dirty(); self }
}
impl<'a> Stylable for Accordion<'a> { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl<'a> Component for Accordion<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.get_all() }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(self.view.core.get_style_mut().to_taffy(), &[]).unwrap();
            self.view.core.set_node(SceneNode::from(new_node)); new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        if self.logic.is_expanded.get() { self.logic.children.layout_all(taffy, measurer, node); } else { taffy.set_children(node, &[]).unwrap(); }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap(); let style = self.view.core.style.borrow();
        if let Some(color) = style.background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw); }
        renderer.draw_text(&self.logic.title, global_pos.x + 12.0, global_pos.y + 8.0, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
        if self.logic.is_expanded.get() { self.logic.children.paint_all(renderer, taffy, node, is_group_hovered, global_pos + Vec2::new(0.0, 32.0), 0); }
    }
    fn on_click(&self, ev: &mut UIEvent) { if ev.local_pos.y < 32.0 { self.logic.is_expanded.update(|v| *v = !*v); } }
    fn on_scroll(&self, ev: &mut UIEvent, d: f32) { if self.logic.is_expanded.get() { self.logic.children.list.iter().for_each(|c| c.on_scroll(ev, d)); } }
    fn on_drag(&self, ev: &mut UIEvent, d: Vec2) { if self.logic.is_expanded.get() { self.logic.children.list.iter().for_each(|c| c.on_drag(ev, d)); } }
}
