use crate::utils::{Style, generate_id, Theme, Color, Accessibility, Attributes, Signal, TextAlign, Vec2, Display, FlexDirection, JustifyContent};
use crate::core::component::Component;
use crate::elements::layout::container::Children;
use crate::elements::text::Text;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- CARD ---

pub struct CardLogic<'a> {
    pub header: Option<Box<dyn Component + 'a>>,
    pub body: Children<'a>,
    pub footer: Option<Box<dyn Component + 'a>>,
    pub accessibility: Accessibility,
}

pub struct CardView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Card<'a> {
    pub id: String,
    pub logic: CardLogic<'a>,
    pub view: CardView,
}

impl<'a> Card<'a> {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        style.background.color = Some(Color::Semantic("surface".into(), None));
        style.padding = crate::utils::Spacing::all(16.0);
        style.rounding = crate::utils::Rounding::all(8.0);

        Self {
            id: generate_id(),
            logic: CardLogic {
                header: None,
                body: Children::new(),
                footer: None,
                accessibility: Accessibility::default(),
            },
            view: CardView {
                style: RefCell::new(style),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }

    pub fn header(mut self, h: Box<dyn Component + 'a>) -> Self { self.logic.header = Some(h); self.view.dirty.store(true, Ordering::Relaxed); self }
    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self { self.logic.body.add(c); self.view.dirty.store(true, Ordering::Relaxed); self }
    pub fn footer(mut self, f: Box<dyn Component + 'a>) -> Self { self.logic.footer = Some(f); self.view.dirty.store(true, Ordering::Relaxed); self }
}

impl<'a> Stylable for Card<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl<'a> Component for Card<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> {
        let mut c = Vec::new();
        if let Some(ref h) = self.logic.header { c.push(h.as_ref()); }
        c.extend(self.logic.body.get_all());
        if let Some(ref f) = self.logic.footer { c.push(f.as_ref()); }
        c
    }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, self.view.style.borrow().to_taffy()).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_with_children(self.view.style.borrow().to_taffy(), &[]).unwrap();
            self.set_node(new_node);
            new_node
        };

        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }

        let mut child_nodes = Vec::new();
        if let Some(ref h) = self.logic.header { child_nodes.push(h.layout(taffy, Some(node))); }
        child_nodes.extend(self.logic.body.layout_all(taffy, node));
        if let Some(ref f) = self.logic.footer { child_nodes.push(f.layout(taffy, Some(node))); }
        
        taffy.set_children(node, &child_nodes).unwrap();
        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style = self.view.style.borrow();
        if let Some(color) = style.background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw);
        }
        
        let children_nodes = taffy.children(node).unwrap();
        let mut idx = 0;
        if let Some(ref h) = self.logic.header { 
            if let Some(n) = children_nodes.get(idx) { 
                let child_layout = taffy.layout(*n).unwrap();
                h.paint(renderer, taffy, *n, false, render_pass, global_pos + Vec2::new(child_layout.location.x, child_layout.location.y)); 
                idx += 1; 
            } 
        }
        self.logic.body.paint_all(renderer, taffy, node, is_group_hovered, render_pass, global_pos, idx);
        
        if let Some(ref f) = self.logic.footer {
            if let Some(n) = children_nodes.last() {
                let child_layout = taffy.layout(*n).unwrap();
                f.paint(renderer, taffy, *n, false, render_pass, global_pos + Vec2::new(child_layout.location.x, child_layout.location.y));
            }
        }
    }
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, event: &mut UIEvent, d: f32) { self.logic.body.list.iter().for_each(|c| c.on_scroll(event, d)); }
    fn on_drag(&self, event: &mut UIEvent, d: Vec2) { self.logic.body.list.iter().for_each(|c| c.on_drag(event, d)); }
}

// --- TABLE ---

pub struct TableRow { pub cells: Vec<String> }

pub struct TableLogic<'a> {
    pub headers: Vec<String>,
    pub rows: Vec<TableRow>,
    pub children: Children<'a>, // For actual rendering nodes
}

pub struct TableView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Table<'a> {
    pub id: String,
    pub logic: TableLogic<'a>,
    pub view: TableView,
}

impl<'a> Table<'a> {
    pub fn new(headers: Vec<String>, rows: Vec<TableRow>) -> Self {
        let mut style = Style::default();
        style.layout.display = Display::Flex;
        style.flex.flex_direction = FlexDirection::Col;
        style.flex.gap = Some(8.0);

        Self {
            id: generate_id(),
            logic: TableLogic {
                headers,
                rows,
                children: Children::new(),
            },
            view: TableView {
                style: RefCell::new(style),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
}

impl<'a> Stylable for Table<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl<'a> Component for Table<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.get_all() }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, self.view.style.borrow().to_taffy()).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_with_children(self.view.style.borrow().to_taffy(), &[]).unwrap();
            self.set_node(new_node);
            new_node
        };
        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.logic.children.layout_all(taffy, node);
        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        if let Some(color) = self.view.style.borrow().background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), 0.0);
        }
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered, render_pass, global_pos, 0);
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, event: &mut UIEvent, d: f32) { self.logic.children.list.iter().for_each(|c| c.on_scroll(event, d)); }
    fn on_drag(&self, event: &mut UIEvent, d: Vec2) { self.logic.children.list.iter().for_each(|c| c.on_drag(event, d)); }
}

// --- ACCORDION ---

pub struct AccordionLogic<'a> {
    pub title: String,
    pub is_expanded: Signal<bool>,
    pub children: Children<'a>,
}

pub struct AccordionView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Accordion<'a> {
    pub id: String,
    pub logic: AccordionLogic<'a>,
    pub view: AccordionView,
}

impl<'a> Accordion<'a> {
    pub fn new(title: impl Into<String>, expanded: Signal<bool>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        style.layout.display = Display::Flex;
        style.flex.flex_direction = FlexDirection::Col;
        style.background.color = Some(Color::Semantic("surface".into(), None));
        style.rounding = crate::utils::Rounding::all(4.0);

        Self {
            id: generate_id(),
            logic: AccordionLogic {
                title: title.into(),
                is_expanded: expanded,
                children: Children::new(),
            },
            view: AccordionView {
                style: RefCell::new(style),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self { self.logic.children.add(c); self.view.dirty.store(true, Ordering::Relaxed); self }
}

impl<'a> Stylable for Accordion<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl<'a> Component for Accordion<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.get_all() }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, self.view.style.borrow().to_taffy()).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_with_children(self.view.style.borrow().to_taffy(), &[]).unwrap();
            self.set_node(new_node);
            new_node
        };
        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        
        if self.logic.is_expanded.get() { 
            self.logic.children.layout_all(taffy, node); 
        } else { 
            taffy.set_children(node, &[]).unwrap(); 
        }

        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style = self.view.style.borrow();
        if let Some(color) = style.background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw);
        }
        
        renderer.draw_text(&self.logic.title, global_pos.x + 12.0, global_pos.y + 8.0, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
        
        if self.logic.is_expanded.get() { 
            self.logic.children.paint_all(renderer, taffy, node, is_group_hovered, render_pass, global_pos + Vec2::new(0.0, 32.0), 0); 
        }
    }

    fn on_click(&self, event: &mut UIEvent) { 
        if event.local_pos.y < 32.0 {
            self.logic.is_expanded.update(|v| *v = !*v); 
        }
    }
    fn on_scroll(&self, event: &mut UIEvent, d: f32) { if self.logic.is_expanded.get() { self.logic.children.list.iter().for_each(|c| c.on_scroll(event, d)); } }
    fn on_drag(&self, event: &mut UIEvent, d: Vec2) { if self.logic.is_expanded.get() { self.logic.children.list.iter().for_each(|c| c.on_drag(event, d)); } }
}
