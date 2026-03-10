use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Theme, Attributes};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

// --- BRAND ---

/// A specialized wrapper for brand identities.
pub struct Brand {
    pub id: String,
    pub name: String,
    pub view: Arc<ViewCore>,
}

impl Brand {
    pub fn new(name: impl Into<String>) -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            name: name.into(),
            view,
        }
    }
}

impl Stylable for Brand {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

impl Component for Brand {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "brand".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("name", self.name.clone());
                attr
            },
            motion: None,
            children: vec![VNode::text(self.name.clone())],
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

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {
    }
}
