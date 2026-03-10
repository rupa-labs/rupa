use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Theme, Attributes};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

// --- BRAND ---

pub struct BrandLogic {
    pub name: String,
}

pub struct BrandView {
    pub core: Arc<ViewCore>,
}

pub struct Brand {
    pub id: String,
    pub logic: BrandLogic,
    pub view: BrandView,
}

impl Brand {
    pub fn new(name: impl Into<String>) -> Self {
        let core = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut core.style());
        Self {
            id: generate_id(),
            logic: BrandLogic { name: name.into() },
            view: BrandView { core },
        }
    }
}

impl Stylable for Brand {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

impl Component for Brand {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "brand".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("name", self.logic.name.clone());
                attr
            },
            children: vec![VNode::text(self.logic.name.clone())],
            key: Some(self.id.clone()),
        })
    }

    fn as_any(&self) -> Option<&dyn std::any::Any> { None }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(style).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent {
            let cur = taffy.children(p).unwrap_or_default();
            if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {
    }
}
