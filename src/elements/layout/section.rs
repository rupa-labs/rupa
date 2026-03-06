use crate::utils::{Style, StyleModifier, generate_id, Theme, Vec2};
use crate::core::component::Component;
use crate::elements::layout::container::Children;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- LOGIC ---

pub struct SectionLogic<'a> {
    pub name: String,
    pub children: Children<'a>,
}

// --- VIEW ---

pub struct SectionView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

// --- COMPONENT ---

pub struct Section<'a> {
    pub id: String,
    pub logic: SectionLogic<'a>,
    pub view: SectionView,
}

impl<'a> Section<'a> {
    pub fn new(name: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: generate_id(),
            logic: SectionLogic {
                name: name.into(),
                children: Children::new(),
            },
            view: SectionView {
                style: RefCell::new(style),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.logic.children.add(child); self.view.dirty.store(true, Ordering::Relaxed); self }
}

impl<'a> Stylable for Section<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> {
        self.view.style.borrow_mut()
    }
}

impl<'a> Component for Section<'a> {
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
    
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.style.borrow();
        let style = if is_group_hovered && style_ref.group_hover.is_some() { 
            style_ref.group_hover.as_ref().unwrap() 
        } else { &style_ref };
        
        if let Some(color) = style.background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw);
        }
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered || style.is_group, render_pass, global_pos, 0);
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _delta: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _delta: Vec2) {}
}
