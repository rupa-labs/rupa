use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Theme, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

pub struct ContainerLogic<'a> {
    pub children: Children<'a>,
}

pub struct ContainerView {
    pub core: Arc<ViewCore>,
}

pub struct Container<'a> {
    pub id: String,
    pub logic: ContainerLogic<'a>,
    pub view: ContainerView,
}

impl<'a> Container<'a> {
    pub fn new() -> Self {
        let core = Arc::new(ViewCore::new());
        // Apply default theme styling if necessary
        Theme::current().apply_defaults(&mut core.style());
        
        Self {
            id: generate_id(),
            logic: ContainerLogic {
                children: Children::new(),
            },
            view: ContainerView { core },
        }
    }

    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self {
        self.logic.children.push(child);
        self.view.core.mark_dirty();
        self
    }
}

impl<'a> Component for Container<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "container".to_string(),
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
            if self.view.core.is_dirty() {
                taffy.set_style(existing.raw(), self.view.core.style().to_taffy()).unwrap();
            }
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

impl<'a> Stylable for Container<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
