use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Theme, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

// --- MODAL ---

pub struct ModalLogic<'a> {
    pub children: Children<'a>,
}

pub struct ModalView {
    pub core: Arc<ViewCore>,
}

pub struct Modal<'a> {
    pub id: String,
    pub logic: ModalLogic<'a>,
    pub view: ModalView,
}

impl<'a> Modal<'a> {
    pub fn new() -> Self {
        let core = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut core.style());
        Self {
            id: generate_id(),
            logic: ModalLogic { children: Children::new() },
            view: ModalView { core },
        }
    }

    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self {
        self.logic.children.push(child);
        self.view.core.mark_dirty();
        self
    }
}

impl<'a> Component for Modal<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    fn is_modal(&self) -> bool { true }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "modal".to_string(),
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
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(style, &[]).unwrap();
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

impl<'a> Stylable for Modal<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- TOOLTIP ---

pub struct TooltipLogic {
    pub text: String,
}

pub struct TooltipView {
    pub core: Arc<ViewCore>,
}

pub struct Tooltip {
    pub id: String,
    pub logic: TooltipLogic,
    pub view: TooltipView,
}

impl Tooltip {
    pub fn new(text: impl Into<String>) -> Self {
        let core = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut core.style());
        Self {
            id: generate_id(),
            logic: TooltipLogic { text: text.into() },
            view: TooltipView { core },
        }
    }
}

impl Component for Tooltip {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "tooltip".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("text", self.logic.text.clone());
                attr
            },
            children: vec![VNode::text(self.logic.text.clone())],
            key: Some(self.id.clone()),
        })
    }

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

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}

impl Stylable for Tooltip {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
