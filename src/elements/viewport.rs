use crate::utils::{Style, generate_id, Signal, Vec2};
use crate::core::component::Component;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- LOGIC ---

pub struct ViewportLogic {
    pub content: Box<dyn Component>,
    pub offset: Signal<Vec2>,
    pub zoom: Signal<f32>,
    pub zoomable: bool,
    pub pannable: bool,
}

// --- VIEW ---

pub struct ViewportView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

// --- COMPONENT ---

pub struct Viewport {
    pub id: String,
    pub logic: ViewportLogic,
    pub view: ViewportView,
}

impl Viewport {
    pub fn new(content: Box<dyn Component>) -> Self {
        Self {
            id: generate_id(),
            logic: ViewportLogic {
                content,
                offset: Signal::new(Vec2::zero()),
                zoom: Signal::new(1.0),
                zoomable: true,
                pannable: true,
            },
            view: ViewportView {
                style: RefCell::new(Style::default()),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn zoomable(mut self, enabled: bool) -> Self { self.logic.zoomable = enabled; self }
    pub fn pannable(mut self, enabled: bool) -> Self { self.logic.pannable = enabled; self }
}

impl Stylable for Viewport {
    fn get_style_mut(&self) -> RefMut<'_, Style> {
        self.view.style.borrow_mut()
    }
}

impl Component for Viewport {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![self.logic.content.as_ref()] }
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

        self.logic.content.layout(taffy, Some(node));
        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let old_offset = renderer.camera_offset; 
        let old_zoom = renderer.camera_zoom;
        
        renderer.camera_offset = self.logic.offset.get(); 
        renderer.camera_zoom = self.logic.zoom.get();
        
        let children = taffy.children(node).unwrap();
        if let Some(content_node) = children.get(0) { 
            self.logic.content.paint(renderer, taffy, *content_node, is_group_hovered, render_pass, global_pos); 
        }
        
        renderer.camera_offset = old_offset; 
        renderer.camera_zoom = old_zoom;
    }

    fn on_click(&self, event: &mut UIEvent) { self.logic.content.on_click(event); }
    fn on_scroll(&self, _event: &mut UIEvent, delta: f32) { 
        if self.logic.zoomable { 
            self.logic.zoom.update(|z| { *z += delta * 0.001; *z = z.clamp(0.1, 10.0); }); 
        } 
    }
    fn on_drag(&self, _event: &mut UIEvent, delta: Vec2) { 
        if self.logic.pannable { 
            let zoom = self.logic.zoom.get();
            self.logic.offset.update(|o| { o.x += delta.x / zoom; o.y += delta.y / zoom; }); 
        } 
    }
}
