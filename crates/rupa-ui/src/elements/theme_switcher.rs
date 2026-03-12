use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, Id, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Attributes};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

/// A ready-to-use toggle for Light and Dark modes.
pub struct ThemeSwitcher {
    pub id: String,
    pub view: Arc<ViewCore>,
}

impl ThemeSwitcher {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl Component for ThemeSwitcher {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { handlers: Default::default(), 
            tag: "theme-switcher".to_string(),
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

impl Stylable for ThemeSwitcher {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
