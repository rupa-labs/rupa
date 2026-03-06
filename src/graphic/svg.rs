use crate::utils::{Style, generate_id, StyleModifier, Vec2, Attributes, Accessibility, Theme};
use crate::core::component::Component;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Path { 
    pub points: Vec<Vec2>, 
    pub color: [f32; 4], 
    pub thickness: f32 
}

pub struct Icon {
    pub id: String,
    pub name: String,
    pub size: f32,
    pub color: [f32; 4],
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

impl Icon {
    pub fn new(name: impl Into<String>) -> Self {
        Self { 
            id: generate_id(), 
            name: name.into(), 
            size: 24.0, 
            color: [1.0, 1.0, 1.0, 1.0],
            node: Cell::new(None),
            dirty: AtomicBool::new(true),
        }
    }
    pub fn size(mut self, s: f32) -> Self { self.size = s; self.mark_dirty(); self }
    pub fn color(mut self, c: [f32; 4]) -> Self { self.color = c; self }
}

impl Component for Icon {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<NodeId> { self.node.get() }
    fn set_node(&self, node: NodeId) { self.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let mut s = taffy::style::Style::default();
        s.size = Size { width: length(self.size), height: length(self.size) };
        
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, s).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_leaf(s).unwrap();
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
    
    fn paint(&self, renderer: &mut Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        renderer.draw_rect(global_pos.x, global_pos.y, self.size, self.size, self.color, 2.0);
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _d: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _d: Vec2) {}
}

pub struct SvgCanvas {
    pub id: String, 
    pub paths: Vec<Path>, 
    pub style: RefCell<Style>, 
    pub attributes: Attributes, 
    pub accessibility: Accessibility,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

impl SvgCanvas {
    pub fn new() -> Self {
        let mut style = Style::default(); 
        Theme::current().apply_defaults(&mut style);
        Self { 
            id: generate_id(), 
            paths: Vec::new(), 
            style: RefCell::new(style), 
            attributes: Attributes::default(), 
            accessibility: Accessibility::default(),
            node: Cell::new(None),
            dirty: AtomicBool::new(true),
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn add_path(&mut self, path: Path) { self.paths.push(path); self.mark_dirty(); }
}

impl Stylable for SvgCanvas {
    fn get_style_mut(&self) -> &mut Style {
        unsafe { &mut *self.style.as_ptr() }
    }
}

impl Component for SvgCanvas {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<NodeId> { self.node.get() }
    fn set_node(&self, node: NodeId) { self.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, self.style.borrow().to_taffy()).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_leaf(self.style.borrow().to_taffy()).unwrap();
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
    
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.style.borrow();
        let style = if is_group_hovered && style_ref.group_hover.is_some() { 
            style_ref.group_hover.as_ref().unwrap() 
        } else { &style_ref };
        
        if let Some(color) = style.background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), 0.0);
        }
        for path in &self.paths {
            for point in &path.points {
                renderer.draw_rect(global_pos.x + point.x, global_pos.y + point.y, path.thickness, path.thickness, path.color, 0.0);
            }
        }
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _d: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _d: Vec2) {}
}
