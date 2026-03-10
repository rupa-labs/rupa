use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_vnode::{Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- SHOW ---

pub struct ShowLogic<'a> {
    pub when: bool,
    pub child: Box<dyn Component + 'a>,
}

pub struct ShowView {
    pub core: ViewCore,
}

pub struct Show<'a> {
    pub id: String,
    pub logic: ShowLogic<'a>,
    pub view: ShowView,
}

impl<'a> Show<'a> {
    pub fn new(when: bool, child: Box<dyn Component + 'a>) -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: ShowLogic { when, child },
            view: ShowView { core: view },
        }
    }
}

impl<'a> Component for Show<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> {
        if self.logic.when { vec![self.logic.child.as_ref()] } else { vec![] }
    }
    
    fn render(&self) -> VNode {
        if self.logic.when {
            self.logic.child.render()
        } else {
            VNode::Empty
        }
    }

    fn as_any(&self) -> Option<&dyn std::any::Any> { None }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(style, &[]).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent {
            let cur = taffy.children(p).unwrap_or_default();
            if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        if self.logic.when {
            let child_node = self.logic.child.layout(taffy, measurer, Some(node));
            taffy.set_children(node, &[child_node]).unwrap();
        } else {
            taffy.set_children(node, &[]).unwrap();
        }
        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        if self.logic.when {
            self.logic.child.paint(renderer, taffy, node, is_group_hovered, global_pos);
        }
    }
}

impl<'a> Stylable for Show<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- FOREACH ---

pub struct ForEachLogic<'a, T> {
    pub items: Vec<T>,
    pub builder: Box<dyn Fn(&T) -> Box<dyn Component + 'a> + Send + Sync>,
}

pub struct ForEachView {
    pub core: ViewCore,
}

pub struct ForEach<'a, T> {
    pub id: String,
    pub logic: ForEachLogic<'a, T>,
    pub view: ForEachView,
}

impl<'a, T> ForEach<'a, T> {
    pub fn new(items: Vec<T>, builder: impl Fn(&T) -> Box<dyn Component + 'a> + Send + Sync + 'static) -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: ForEachLogic { items, builder: Box::new(builder) },
            view: ForEachView { core: view },
        }
    }
}

impl<'a, T: Send + Sync> Component for ForEach<'a, T> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    
    fn render(&self) -> VNode {
        let children = self.logic.items.iter()
            .map(|item| (self.logic.builder)(item).render())
            .collect();
        VNode::Fragment(children)
    }

    fn as_any(&self) -> Option<&dyn std::any::Any> { None }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(style, &[]).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent {
            let cur = taffy.children(p).unwrap_or_default();
            if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        
        let child_nodes: Vec<NodeId> = self.logic.items.iter()
            .map(|item| (self.logic.builder)(item).layout(taffy, measurer, Some(node)))
            .collect();
        
        taffy.set_children(node, &child_nodes).unwrap();
        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        for item in &self.logic.items {
            let child = (self.logic.builder)(item);
            child.paint(renderer, taffy, node, is_group_hovered, global_pos);
        }
    }
}

impl<'a, T: Send + Sync> Stylable for ForEach<'a, T> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
