use crate::utils::{Style, generate_id, Signal, Theme, Color, Accessibility, Attributes, Role, TextAlign, Vec2};
use crate::core::component::Component;
use crate::elements::layout::container::Children;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- MODAL ---

pub struct ModalLogic<'a> {
    pub is_open: Signal<bool>,
    pub children: Children<'a>,
    pub header: Option<Box<dyn Component + 'a>>,
    pub footer: Option<Box<dyn Component + 'a>>,
    pub accessibility: Accessibility,
}

pub struct ModalView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Modal<'a> {
    pub id: String,
    pub logic: ModalLogic<'a>,
    pub view: ModalView,
}

impl<'a> Modal<'a> {
    pub fn new(is_open: Signal<bool>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        style.background.color = Some(Color::Semantic("surface".into(), None));
        style.sizing.width = Some(400.0);
        style.padding = crate::utils::Spacing::all(24.0);

        Self {
            id: generate_id(),
            logic: ModalLogic {
                is_open,
                children: Children::new(),
                header: None,
                footer: None,
                accessibility: Accessibility { role: Role::Alert, ..Default::default() },
            },
            view: ModalView {
                style: RefCell::new(style),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn header(mut self, h: Box<dyn Component + 'a>) -> Self { self.logic.header = Some(h); self.view.dirty.store(true, Ordering::Relaxed); self }
    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self { self.logic.children.add(c); self.view.dirty.store(true, Ordering::Relaxed); self }
    pub fn footer(mut self, f: Box<dyn Component + 'a>) -> Self { self.logic.footer = Some(f); self.view.dirty.store(true, Ordering::Relaxed); self }
}

impl<'a> Stylable for Modal<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl<'a> Component for Modal<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> {
        let mut c = Vec::new();
        if let Some(ref h) = self.logic.header { c.push(h.as_ref()); }
        c.extend(self.logic.children.get_all());
        if let Some(ref f) = self.logic.footer { c.push(f.as_ref()); }
        c
    }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        if !self.logic.is_open.get() { 
            return taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::None, ..Default::default() }).unwrap(); 
        }
        
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
        child_nodes.extend(self.logic.children.layout_all(taffy, node));
        if let Some(ref f) = self.logic.footer { child_nodes.push(f.layout(taffy, Some(node))); }
        
        taffy.set_children(node, &child_nodes).unwrap();
        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        if !self.logic.is_open.get() { return; }
        let layout = taffy.layout(node).unwrap();
        if let Some(color) = self.view.style.borrow().background.color.clone() { 
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), 12.0); 
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
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered, render_pass, global_pos, idx);
        
        if let Some(ref f) = self.logic.footer {
            if let Some(n) = children_nodes.last() {
                let child_layout = taffy.layout(*n).unwrap();
                f.paint(renderer, taffy, *n, false, render_pass, global_pos + Vec2::new(child_layout.location.x, child_layout.location.y));
            }
        }
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, event: &mut UIEvent, d: f32) { if self.logic.is_open.get() { self.logic.children.list.iter().for_each(|c| c.on_scroll(event, d)); } }
    fn on_drag(&self, event: &mut UIEvent, d: Vec2) { if self.logic.is_open.get() { self.logic.children.list.iter().for_each(|c| c.on_drag(event, d)); } }
}

// --- TOOLTIP ---

pub struct TooltipLogic {
    pub text: String,
}

pub struct TooltipView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Tooltip {
    pub id: String,
    pub logic: TooltipLogic,
    pub view: TooltipView,
}

impl Tooltip {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            logic: TooltipLogic { text: text.into() },
            view: TooltipView {
                style: RefCell::new(Style::default()),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
}

impl Stylable for Tooltip {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl Component for Tooltip {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
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
            let new_node = taffy.new_leaf(self.view.style.borrow().to_taffy()).unwrap();
            self.set_node(new_node);
            new_node
        };
        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.1, 0.1, 0.1, 0.9], 4.0);
        renderer.draw_text(&self.logic.text, global_pos.x + 4.0, global_pos.y + 2.0, 12.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}
