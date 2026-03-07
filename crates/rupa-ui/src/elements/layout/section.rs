use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_styling::{Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use crate::elements::Text;
use crate::elements::layout::VStack;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

pub struct Section<'a> {
    pub id: String,
    pub title: String,
    pub children: Children<'a>,
    pub view: ViewCore,
}

impl<'a> Section<'a> {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            title: title.into(),
            children: Children::new(),
            view: ViewCore::new(),
        }
    }

    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self {
        self.children.push(child);
        self
    }
}

impl<'a> Component for Section<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "section".to_string(),
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
        let node = if let Some(existing) = self.view.get_node() {
            if self.view.is_dirty() { taffy.set_style(existing.raw(), self.view.style().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(self.view.style().to_taffy(), &[]).unwrap();
            self.view.set_node(SceneNode::from(new_node));
            new_node
        };

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

impl<'a> Stylable for Section<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
