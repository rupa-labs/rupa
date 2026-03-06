use crate::core::component::Component;
use crate::renderer::renderer::Renderer;
use crate::utils::vector::Vec2;
use crate::utils::{Style, generate_id, StyleModifier, Accessibility, Attributes, Overflow, Signal, Theme};
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Children<'a> { 
    pub list: Vec<Box<dyn Component + 'a>> 
}

impl<'a> Children<'a> {
    pub fn new() -> Self { Self { list: Vec::new() } }
    
    pub fn add(&mut self, child: Box<dyn Component + 'a>) { 
        self.list.push(child); 
    }
    
    pub fn get_all(&self) -> Vec<&dyn Component> {
        self.list.iter().map(|c| c.as_ref()).collect()
    }

    pub fn layout_all(&self, taffy: &mut TaffyTree<()>, parent: NodeId) -> Vec<NodeId> { 
        let child_nodes: Vec<NodeId> = self.list.iter().map(|child| child.layout(taffy, Some(parent))).collect();
        taffy.set_children(parent, &child_nodes).unwrap();
        child_nodes
    }
    
    pub fn paint_all(
        &self, 
        renderer: &mut Renderer, 
        taffy: &TaffyTree<()>, 
        parent_node: NodeId, 
        is_group_hovered: bool, 
        render_pass: &mut wgpu::RenderPass<'_>, 
        parent_global_pos: Vec2, 
        start_index: usize
    ) {
        if let Ok(children_nodes) = taffy.children(parent_node) {
            for (i, child) in self.list.iter().enumerate() {
                if let Some(node) = children_nodes.get(i + start_index) {
                    if let Ok(layout) = taffy.layout(*node) {
                        let global_pos = parent_global_pos + Vec2::new(layout.location.x, layout.location.y);
                        child.paint(renderer, taffy, *node, is_group_hovered, render_pass, global_pos);
                    }
                }
            }
        }
    }
}

// --- LOGIC ---

pub struct ContainerLogic<'a> {
    pub children: Children<'a>,
    pub scroll_offset: Signal<Vec2>,
    pub accessibility: Accessibility,
    pub attributes: Attributes,
}

// --- VIEW ---

pub struct ContainerView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

// --- COMPONENT ---

pub struct Container<'a> {
    pub id: String, 
    pub logic: ContainerLogic<'a>,
    pub view: ContainerView,
}

impl<'a> Container<'a> {
    pub fn new() -> Self {
        let mut style = Style::default(); 
        Theme::current().apply_defaults(&mut style);
        
        Self { 
            id: generate_id(), 
            logic: ContainerLogic {
                children: Children::new(), 
                scroll_offset: Signal::new(Vec2::zero()),
                accessibility: Accessibility::default(), 
                attributes: Attributes::default(), 
            },
            view: ContainerView {
                style: RefCell::new(style), 
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.logic.children.add(child); self.view.dirty.store(true, Ordering::Relaxed); self }
}

impl<'a> Stylable for Container<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> {
        self.view.style.borrow_mut()
    }
}

impl<'a> Component for Container<'a> {
    fn id(&self) -> &str { &self.id }
    
    fn children(&self) -> Vec<&dyn Component> {
        self.logic.children.get_all()
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

        self.logic.children.layout_all(taffy, node);
        self.clear_dirty();
        node
    }
    
    fn paint(
        &self, 
        renderer: &mut Renderer, 
        taffy: &TaffyTree<()>, 
        node: NodeId, 
        is_group_hovered: bool, 
        render_pass: &mut wgpu::RenderPass<'_>, 
        global_pos: Vec2
    ) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.style.borrow();
        
        let style: &Style = if is_group_hovered && style_ref.group_hover.is_some() { 
            style_ref.group_hover.as_ref().unwrap() 
        } else { 
            &style_ref 
        };
        
        if let Some(color) = style.background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw);
        }

        let needs_clip = style.layout.overflow_x != Overflow::Visible || style.layout.overflow_y != Overflow::Visible;
        if needs_clip { 
            renderer.push_clip(global_pos.x, global_pos.y, layout.size.width, layout.size.height, render_pass); 
        }
        
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered || style.is_group, render_pass, global_pos + self.logic.scroll_offset.get(), 0);
        
        if needs_clip { renderer.pop_clip(render_pass); }
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, delta: f32) { 
        if self.view.style.borrow().layout.overflow_y == Overflow::Scroll { 
            self.logic.scroll_offset.update(|o| o.y += delta); 
        } 
    }
    fn on_drag(&self, _event: &mut UIEvent, delta: Vec2) { 
        let style = self.view.style.borrow();
        if style.layout.overflow_x == Overflow::Scroll || style.layout.overflow_y == Overflow::Scroll { 
            self.logic.scroll_offset.update(|o| { o.x += delta.x; o.y += delta.y; }); 
        } 
    }
}
