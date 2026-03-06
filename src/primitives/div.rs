use crate::utils::{Style, generate_id, Attributes, Theme, Accessibility, Overflow, Vec2, Signal, StyleModifier};
use crate::core::component::Component;
use crate::elements::layout::container::Children;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Div<'a> {
    pub id: String,
    pub style: RefCell<Style>,
    pub accessibility: Accessibility,
    pub attributes: Attributes,
    pub children: Children<'a>,
    pub scroll_offset: Signal<Vec2>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

impl<'a> Div<'a> {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: generate_id(), 
            style: RefCell::new(style), 
            accessibility: Accessibility::default(),
            attributes: Attributes::default(), 
            children: Children::new(),
            scroll_offset: Signal::new(Vec2::zero()),
            node: Cell::new(None),
            dirty: AtomicBool::new(true),
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style.borrow_mut()); self.mark_dirty(); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.children.add(child); self.mark_dirty(); self }
}

impl<'a> Stylable for Div<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> {
        self.style.borrow_mut()
    }
}

impl<'a> Component for Div<'a> {
    fn id(&self) -> &str { &self.id }
    
    fn children(&self) -> Vec<&dyn Component> {
        self.children.get_all()
    }

    fn get_node(&self) -> Option<NodeId> { self.node.get() }
    fn set_node(&self, node: NodeId) { self.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { 
                taffy.set_style(existing, self.style.borrow().to_taffy()).unwrap(); 
            }
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

        self.children.layout_all(taffy, node);
        self.clear_dirty();
        node
    }
    
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.style.borrow();
        let style: &Style = if is_group_hovered && style_ref.group_hover.is_some() { 
            style_ref.group_hover.as_ref().unwrap() 
        } else { &style_ref };
        
        if let Some(color) = style.background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw);
        }

        let needs_clip = style.layout.overflow_x != Overflow::Visible || style.layout.overflow_y != Overflow::Visible;
        if needs_clip { 
            renderer.push_clip(global_pos.x, global_pos.y, layout.size.width, layout.size.height, render_pass); 
        }
        
        self.children.paint_all(renderer, taffy, node, is_group_hovered || style.is_group, render_pass, global_pos + self.scroll_offset.get(), 0);
        
        if needs_clip { renderer.pop_clip(render_pass); }
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, event: &mut UIEvent, delta: f32) { 
        if self.style.borrow().layout.overflow_y == Overflow::Scroll { 
            self.scroll_offset.update(|o| o.y += delta); 
        } 
        self.children.list.iter().for_each(|c| c.on_scroll(event, delta));
    }
    fn on_drag(&self, event: &mut UIEvent, delta: Vec2) { 
        let style = self.style.borrow();
        if style.layout.overflow_x == Overflow::Scroll || style.layout.overflow_y == Overflow::Scroll { 
            self.scroll_offset.update(|o| { o.x += delta.x; o.y += delta.y; }); 
        } 
        self.children.list.iter().for_each(|c| c.on_drag(event, delta));
    }
}
