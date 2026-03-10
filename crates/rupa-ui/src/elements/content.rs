use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Theme, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

// --- CARD ---

/// A container component with an elevated surface.
pub struct Card<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub view: Arc<ViewCore>,
}

impl<'a> Card<'a> {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            children: Children::new(),
            view,
        }
    }

    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self {
        self.children.push(child);
        self.view.mark_dirty();
        self
    }
}

impl<'a> Component for Card<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "card".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: self.children.render_all(),
            key: Some(self.id.clone()),
        })
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.view.style().to_taffy(), &[]).unwrap();
        self.view.set_node(SceneNode::from(node));
        let child_nodes = self.children.layout_all(taffy, measurer);
        taffy.set_children(node, &child_nodes).unwrap();
        self.view.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let style_ref = self.view.style.read().unwrap();
        if let Some(ref color) = style_ref.background.color {
            let layout = taffy.layout(node).unwrap();
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            );
        }
        self.children.paint_all(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos, 0);
    }
}

impl<'a> Stylable for Card<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- TABLE ---

/// A component for displaying structured data.
pub struct Table<'a> {
    pub id: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<Box<dyn Component + 'a>>>,
    pub view: Arc<ViewCore>,
}

impl<'a> Table<'a> {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            headers: vec![],
            rows: vec![],
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl<'a> Component for Table<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "table".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: vec![],
            key: Some(self.id.clone()),
        })
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.view.style().to_taffy()).unwrap();
        self.view.set_node(SceneNode::from(node));
        self.view.clear_dirty();
        node
    }

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}

impl<'a> Stylable for Table<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- ACCORDION ---

/// A space-saving collapsible content component.
pub struct Accordion<'a> {
    pub id: String,
    pub items: Vec<(String, Box<dyn Component + 'a>)>,
    pub expanded_index: Signal<Option<usize>>,
    pub view: Arc<ViewCore>,
}

impl<'a> Accordion<'a> {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            items: vec![],
            expanded_index: Signal::new(None),
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl<'a> Component for Accordion<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "accordion".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: vec![],
            key: Some(self.id.clone()),
        })
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.view.style().to_taffy()).unwrap();
        self.view.set_node(SceneNode::from(node));
        self.view.clear_dirty();
        node
    }

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}

impl<'a> Stylable for Accordion<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
