use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

/// A layout container with constrained dimensions.
pub struct Container<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub view: Arc<ViewCore>,
}

impl<'a> Container<'a> {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            children: Children::new(),
            view: Arc::new(ViewCore::new()),
        }
    }

    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self {
        self.children.push(child);
        self.view.mark_dirty();
        self
    }
}

impl<'a> Component for Container<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "container".to_string(),
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
        self.view.set_node(node.into());
        
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

impl<'a> Stylable for Container<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
