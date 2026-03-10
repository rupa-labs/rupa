use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

// --- ROUTER ---

pub struct RouterState {
    pub current_path: Signal<String>,
}

pub struct Router<'a> {
    pub id: String,
    pub state: RouterState,
    pub children: Children<'a>,
    pub view: Arc<ViewCore>,
}

impl<'a> Router<'a> {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            state: RouterState { current_path: Signal::new("/".to_string()) },
            children: Children::new(),
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl<'a> Component for Router<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "router".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
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
        self.children.paint_all(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos, 0);
    }
}

impl<'a> Stylable for Router<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
