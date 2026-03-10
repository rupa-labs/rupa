use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Theme, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

// --- CARD ---

pub struct CardLogic<'a> {
    pub children: Children<'a>,
}

pub struct CardView {
    pub core: Arc<ViewCore>,
}

pub struct Card<'a> {
    pub id: String,
    pub logic: CardLogic<'a>,
    pub view: CardView,
}

impl<'a> Card<'a> {
    pub fn new() -> Self {
        let core = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut core.style());
        Self {
            id: generate_id(),
            logic: CardLogic { children: Children::new() },
            view: CardView { core },
        }
    }
}

impl<'a> Stylable for Card<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

impl<'a> Component for Card<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "card".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            children: self.logic.children.render_all(),
            key: Some(self.id.clone()),
        })
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.style().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(self.view.core.style().to_taffy(), &[]).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        let child_nodes = self.logic.children.layout_all(taffy, measurer);
        taffy.set_children(node, &child_nodes).unwrap();
        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let style_ref = self.view.core.style.read().unwrap();
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos, 0);
    }
}

// --- TABLE ---

pub struct TableLogic<'a> {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<Box<dyn Component + 'a>>>,
}

pub struct TableView {
    pub core: Arc<ViewCore>,
}

pub struct Table<'a> {
    pub id: String,
    pub logic: TableLogic<'a>,
    pub view: TableView,
}

impl<'a> Table<'a> {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            logic: TableLogic { headers: vec![], rows: vec![] },
            view: TableView { core: Arc::new(ViewCore::new()) },
        }
    }
}

impl<'a> Component for Table<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "table".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            children: vec![],
            key: Some(self.id.clone()),
        })
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.view.core.style().to_taffy()).unwrap();
        self.view.core.set_node(SceneNode::from(node));
        node
    }

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}

// --- ACCORDION ---

pub struct AccordionLogic<'a> {
    pub items: Vec<(String, Box<dyn Component + 'a>)>,
    pub expanded_index: Signal<Option<usize>>,
}

pub struct AccordionView {
    pub core: Arc<ViewCore>,
}

pub struct Accordion<'a> {
    pub id: String,
    pub logic: AccordionLogic<'a>,
    pub view: AccordionView,
}

impl<'a> Accordion<'a> {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            logic: AccordionLogic { items: vec![], expanded_index: Signal::new(None) },
            view: AccordionView { core: Arc::new(ViewCore::new()) },
        }
    }
}

impl<'a> Component for Accordion<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "accordion".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            children: vec![],
            key: Some(self.id.clone()),
        })
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.view.core.style().to_taffy()).unwrap();
        self.view.core.set_node(SceneNode::from(node));
        node
    }

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}
