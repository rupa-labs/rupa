use crate::utils::{Style, StyleModifier, generate_id, Theme, Color, Attributes, Accessibility, Vec2, TextAlign};
use crate::core::component::Component;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- LOGIC ---

pub struct TextLogic {
    pub content: String,
    pub attributes: Attributes,
    pub accessibility: Accessibility,
}

// --- VIEW ---

pub struct TextView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

// --- COMPONENT ---

pub struct Text { 
    pub id: String, 
    pub logic: TextLogic,
    pub view: TextView,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        let mut style = Style::default(); 
        Theme::current().apply_defaults(&mut style);
        Self { 
            id: generate_id(), 
            logic: TextLogic {
                content: content.into(), 
                attributes: Attributes::default(), 
                accessibility: Accessibility::default(),
            },
            view: TextView {
                style: RefCell::new(style), 
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
}

impl Stylable for Text {
    fn get_style_mut(&self) -> RefMut<'_, Style> {
        self.view.style.borrow_mut()
    }
}

impl Component for Text {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }
    
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() {
                taffy.set_style(existing, self.view.style.borrow().to_taffy()).unwrap();
            }
            existing
        } else {
            let new_node = taffy.new_leaf(self.view.style.borrow().to_taffy()).unwrap();
            self.set_node(new_node);
            new_node
        };

        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) {
                taffy.add_child(p, node).unwrap();
            }
        }

        self.clear_dirty();
        node
    }
    
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let style_ref = self.view.style.borrow();
        let style = if is_group_hovered && style_ref.group_hover.is_some() { 
            style_ref.group_hover.as_ref().unwrap() 
        } else { &style_ref };
        
        let color = style.typography.color.clone().unwrap_or(Color::Rgba(1.0, 1.0, 1.0, 1.0)).to_rgba();
        renderer.draw_text(&self.logic.content, global_pos.x, global_pos.y, 16.0, color, TextAlign::Left);
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}
