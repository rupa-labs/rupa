use crate::utils::{Style, generate_id, Signal, Vec2, StyleModifier};
use crate::core::component::Component;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Viewport {
    pub id: String, 
    pub style: RefCell<Style>, 
    pub content: Box<dyn Component>, 
    pub offset: Signal<Vec2>, 
    pub zoom: Signal<f32>, 
    pub zoomable: bool, 
    pub pannable: bool,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

impl Viewport {
    pub fn new(content: Box<dyn Component>) -> Self {
        Self { 
            id: generate_id(), 
            style: RefCell::new(Style::default()), 
            content, 
            offset: Signal::new(Vec2::zero()), 
            zoom: Signal::new(1.0), 
            zoomable: true, 
            pannable: true,
            node: Cell::new(None),
            dirty: AtomicBool::new(true),
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn zoomable(mut self, enabled: bool) -> Self { self.zoomable = enabled; self }
    pub fn pannable(mut self, enabled: bool) -> Self { self.pannable = enabled; self }
}

impl Stylable for Viewport {
    fn get_style_mut(&self) -> &mut Style {
        unsafe { &mut *self.style.as_ptr() }
    }
}

impl Component for Viewport {
    fn id(&self) -> &str { &self.id }
    
    fn children(&self) -> Vec<&dyn Component> {
        vec![self.content.as_ref()]
    }

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
            let new_node = taffy.new_with_children(self.style.borrow().to_taffy(), &[]).unwrap();
            self.set_node(new_node);
            new_node
        };

        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }

        self.content.layout(taffy, Some(node));
        self.clear_dirty();
        node
    }
    
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let old_offset = renderer.camera_offset; 
        let old_zoom = renderer.camera_zoom;
        
        renderer.camera_offset = self.offset.get(); 
        renderer.camera_zoom = self.zoom.get();
        
        let children = taffy.children(node).unwrap();
        if let Some(content_node) = children.get(0) { 
            self.content.paint(renderer, taffy, *content_node, is_group_hovered, render_pass, global_pos); 
        }
        
        renderer.camera_offset = old_offset; 
        renderer.camera_zoom = old_zoom;
    }
    
    fn on_click(&self, event: &mut UIEvent) { self.content.on_click(event); }
    
    fn on_scroll(&self, event: &mut UIEvent, delta: f32) { 
        if self.zoomable { 
            self.zoom.update(|z| { *z += delta * 0.001; *z = z.clamp(0.1, 10.0); }); 
        } 
        self.content.on_scroll(event, delta);
    }
    
    fn on_drag(&self, event: &mut UIEvent, delta: Vec2) { 
        if self.pannable { 
            self.offset.update(|o| { o.x += delta.x / self.zoom.get(); o.y += delta.y / self.zoom.get(); }); 
        } 
        self.content.on_drag(event, delta);
    }
}
