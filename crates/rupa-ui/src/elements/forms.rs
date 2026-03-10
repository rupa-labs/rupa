use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Color, Theme, TextAlign, Attributes};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

// --- LABEL ---

/// A semantic label for form elements.
pub struct Label {
    pub id: String,
    pub text: String,
    pub view: Arc<ViewCore>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            text: text.into(),
            view,
        }
    }
}

impl Component for Label {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "label".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("text", self.text.clone());
                attr
            },
            motion: None,
            children: vec![VNode::text(self.text.clone())],
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

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let text_color = Color::Semantic("text".into(), None).to_rgba();
        renderer.draw_text(&self.text, global_pos.x + layout.location.x, global_pos.y + layout.location.y, layout.size.width, 14.0, text_color, TextAlign::Left);
    }
}

impl Stylable for Label {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- INPUT ---

/// A single-line text input field.
pub struct Input {
    pub id: String,
    pub value: Signal<String>,
    pub placeholder: String,
    pub view: Arc<ViewCore>,
}

impl Input {
    pub fn new(placeholder: impl Into<String>) -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            value: Signal::new(String::new()),
            placeholder: placeholder.into(),
            view,
        }
    }
}

impl Component for Input {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "input".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("placeholder", self.placeholder.clone());
                attr.insert("value", self.value.get());
                attr
            },
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

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.style.read().unwrap();
        if let Some(ref color) = style_ref.background.color {
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            );
        }
        // Draw placeholder or value here if needed
    }
}

impl Stylable for Input {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- CHECKBOX ---

/// A classic binary selection toggle.
pub struct Checkbox {
    pub id: String,
    pub checked: Signal<bool>,
    pub view: Arc<ViewCore>,
}

impl Checkbox {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            checked: Signal::new(false),
            view,
        }
    }
}

impl Component for Checkbox {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "checkbox".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("checked", self.checked.get().to_string());
                attr
            },
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

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.style.read().unwrap();
        if let Some(ref color) = style_ref.background.color {
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            );
        }
    }
}

impl Stylable for Checkbox {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- SWITCH ---

/// A modern binary toggle switch.
pub struct Switch {
    pub id: String,
    pub active: Signal<bool>,
    pub view: Arc<ViewCore>,
}

impl Switch {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            active: Signal::new(false),
            view,
        }
    }
}

impl Component for Switch {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "switch".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("active", self.active.get().to_string());
                attr
            },
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

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.style.read().unwrap();
        if let Some(ref color) = style_ref.background.color {
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            );
        }
    }
}

impl Stylable for Switch {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- RADIO ---

/// A radio button for mutually exclusive selection.
pub struct Radio {
    pub id: String,
    pub selected: Signal<bool>,
    pub view: Arc<ViewCore>,
}

impl Radio {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            selected: Signal::new(false),
            view,
        }
    }
}

impl Component for Radio {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "radio".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("selected", self.selected.get().to_string());
                attr
            },
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

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.style.read().unwrap();
        if let Some(ref color) = style_ref.background.color {
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            );
        }
    }
}

impl Stylable for Radio {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- SELECT ---

/// A dropdown selection menu.
pub struct Select {
    pub id: String,
    pub options: Vec<String>,
    pub selected_index: Signal<Option<usize>>,
    pub view: Arc<ViewCore>,
}

impl Select {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            options: vec![],
            selected_index: Signal::new(None),
            view,
        }
    }
}

impl Component for Select {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "select".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("options_count", self.options.len().to_string());
                attr
            },
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

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.style.read().unwrap();
        if let Some(ref color) = style_ref.background.color {
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            );
        }
    }
}

impl Stylable for Select {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
