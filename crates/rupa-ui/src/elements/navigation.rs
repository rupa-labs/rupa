use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_styling::{Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- NAVBAR ---

pub struct NavbarLogic<'a> {
    pub children: Children<'a>,
}

pub struct NavbarView {
    pub core: ViewCore,
}

pub struct Navbar<'a> {
    pub id: String,
    pub logic: NavbarLogic<'a>,
    pub view: NavbarView,
}

impl<'a> Navbar<'a> {
    pub fn new() -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: NavbarLogic {
                children: Children::new(),
            },
            view: NavbarView { core: view },
        }
    }
}

impl<'a> Component for Navbar<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.as_refs() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "navbar".to_string(),
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
            if self.view.core.is_dirty() {
                taffy.set_style(existing.raw(), style).unwrap();
            }
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

impl<'a> Stylable for Navbar<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- TABS ---

pub struct TabsLogic<'a> {
    pub children: Children<'a>,
}

pub struct TabsView {
    pub core: ViewCore,
}

pub struct Tabs<'a> {
    pub id: String,
    pub logic: TabsLogic<'a>,
    pub view: TabsView,
}

impl<'a> Tabs<'a> {
    pub fn new() -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: TabsLogic {
                children: Children::new(),
            },
            view: TabsView { core: view },
        }
    }
}

impl<'a> Component for Tabs<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.as_refs() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "tabs".to_string(),
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
            if self.view.core.is_dirty() {
                taffy.set_style(existing.raw(), style).unwrap();
            }
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

impl<'a> Stylable for Tabs<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- BREADCRUMB ---

pub struct BreadcrumbLogic<'a> {
    pub children: Children<'a>,
}

pub struct BreadcrumbView {
    pub core: ViewCore,
}

pub struct Breadcrumb<'a> {
    pub id: String,
    pub logic: BreadcrumbLogic<'a>,
    pub view: BreadcrumbView,
}

impl<'a> Breadcrumb<'a> {
    pub fn new() -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: BreadcrumbLogic {
                children: Children::new(),
            },
            view: BreadcrumbView { core: view },
        }
    }
}

impl<'a> Component for Breadcrumb<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.as_refs() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "breadcrumb".to_string(),
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
            if self.view.core.is_dirty() {
                taffy.set_style(existing.raw(), style).unwrap();
            }
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

impl<'a> Stylable for Breadcrumb<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
