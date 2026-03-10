use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_vnode::{Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

pub struct GridLogic<'a> {
    pub children: Children<'a>,
}

pub struct GridView {
    pub core: ViewCore,
}

pub struct Grid<'a> {
    pub id: String,
    pub logic: GridLogic<'a>,
    pub view: GridView,
}

impl<'a> Grid<'a> {
    pub fn new() -> Self {
        let view = GridView { core: ViewCore::new() };
        view.core.style().layout.display = rupa_vnode::Display::Grid;
        
        Self {
            id: generate_id(),
            logic: GridLogic { children: Children::new() },
            view,
        }
    }

    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self {
        self.logic.children.push(child);
        self
    }
}

impl<'a> Component for Grid<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.as_refs() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "grid".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            children: self.logic.children.render_all(),
            key: Some(self.id.clone()),
        })
    }


    fn get_node(&self) -> Option<rupa_core::scene::SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: rupa_core::scene::SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = taffy.new_with_children(style, &[]).unwrap();
        self.set_node(node.into());
        
        for child in self.logic.children.iter() {
            let child_node = child.layout(taffy, measurer, Some(node));
            taffy.add_child(node, child_node).unwrap();
        }

        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let style_ref = self.view.core.style.read().unwrap();
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos, 0);
    }
}

impl<'a> Stylable for Grid<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
