use crate::utils::{Signal, generate_id, Vec2};
use crate::core::component::Component;
use crate::renderer::renderer::Renderer;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::fmt::Debug;
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Show<'a> { 
    pub id: String, 
    pub when: Signal<bool>, 
    pub child: Box<dyn Component + 'a>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

impl<'a> Show<'a> { 
    pub fn new(when: Signal<bool>, child: Box<dyn Component + 'a>) -> Self { 
        Self { 
            id: generate_id(), 
            when, 
            child,
            node: Cell::new(None),
            dirty: AtomicBool::new(true),
        } 
    } 
}

impl<'a> Component for Show<'a> {
    fn id(&self) -> &str { &self.id }
    
    fn children(&self) -> Vec<&dyn Component> {
        if self.when.get() { vec![self.child.as_ref()] } else { vec![] }
    }

    fn get_node(&self) -> Option<NodeId> { self.node.get() }
    fn set_node(&self, node: NodeId) { self.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        if self.when.get() { 
            let node = self.child.layout(taffy, parent);
            self.set_node(node);
            node
        } else { 
            let node = if let Some(existing) = self.get_node() {
                taffy.set_style(existing, taffy::style::Style { display: taffy::style::Display::None, ..Default::default() }).unwrap();
                existing
            } else {
                let new_node = taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::None, ..Default::default() }).unwrap();
                self.set_node(new_node);
                new_node
            };
            if let Some(p) = parent {
                let current_children = taffy.children(p).unwrap_or_default();
                if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
            }
            node 
        }
    }
    
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) { 
        if self.when.get() { 
            self.child.paint(renderer, taffy, node, is_group_hovered, render_pass, global_pos); 
        } 
    }
    
    fn on_click(&self, event: &mut UIEvent) { if self.when.get() { self.child.on_click(event); } }
    fn on_scroll(&self, event: &mut UIEvent, delta: f32) { if self.when.get() { self.child.on_scroll(event, delta); } }
    fn on_drag(&self, event: &mut UIEvent, delta: Vec2) { if self.when.get() { self.child.on_drag(event, delta); } }
}

pub struct ForEach<'a, T: Clone + Debug + Send + Sync + 'static> { 
    pub id: String, 
    pub items: Signal<Vec<T>>, 
    pub render_item: Box<dyn Fn(&T) -> Box<dyn Component + 'a> + 'a>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

impl<'a, T: Clone + Debug + Send + Sync + 'static> ForEach<'a, T> { 
    pub fn new(items: Signal<Vec<T>>, render_item: impl Fn(&T) -> Box<dyn Component + 'a> + 'a) -> Self { 
        Self { 
            id: generate_id(), 
            items, 
            render_item: Box::new(render_item),
            node: Cell::new(None),
            dirty: AtomicBool::new(true),
        } 
    } 
}

impl<'a, T: Clone + Debug + Send + Sync + 'static> Component for ForEach<'a, T> {
    fn id(&self) -> &str { &self.id }
    
    fn children(&self) -> Vec<&dyn Component> {
        vec![] // Contextually generated, skipping static list for ForEach
    }

    fn get_node(&self) -> Option<NodeId> { self.node.get() }
    fn set_node(&self, node: NodeId) { self.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            existing
        } else {
            let new_node = taffy.new_with_children(taffy::style::Style::default(), &[]).unwrap();
            self.set_node(new_node);
            new_node
        };

        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }

        let mut child_nodes = Vec::new();
        for item in self.items.get().iter() { 
            child_nodes.push((self.render_item)(item).layout(taffy, Some(node))); 
        }
        taffy.set_children(node, &child_nodes).unwrap();
        
        self.clear_dirty();
        node
    }
    
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let children_nodes = taffy.children(node).unwrap();
        for (i, item) in self.items.get().iter().enumerate() { 
            if let Some(child_node) = children_nodes.get(i) { 
                let cl = taffy.layout(*child_node).unwrap();
                (self.render_item)(item).paint(renderer, taffy, *child_node, is_group_hovered, render_pass, global_pos + Vec2::new(cl.location.x, cl.location.y)); 
            } 
        }
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}
