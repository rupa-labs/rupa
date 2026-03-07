use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_styling::{Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- ICON ---

pub struct IconLogic {
    pub name: String,
}

pub struct IconView {
    pub core: ViewCore,
}

pub struct Icon {
    pub id: String,
    pub logic: IconLogic,
    pub view: IconView,
}

impl Icon {
    pub fn new(name: impl Into<String>) -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: IconLogic { name: name.into() },
            view: IconView { core: view },
        }
    }
}

impl Component for Icon {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "icon".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("name", self.logic.name.clone());
                attr
            },
            children: vec![],
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

impl Stylable for Icon {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- SVG ---

pub struct SvgLogic {
    pub source: String,
}

pub struct SvgView {
    pub core: ViewCore,
}

pub struct Svg {
    pub id: String,
    pub logic: SvgLogic,
    pub view: SvgView,
}

pub type SvgCanvas = Svg;

impl Svg {
    pub fn new(source: impl Into<String>) -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: SvgLogic { source: source.into() },
            view: SvgView { core: view },
        }
    }
}

impl Component for Svg {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "svg".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("source", self.logic.source.clone());
                attr
            },
            children: vec![],
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

impl Stylable for Svg {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
