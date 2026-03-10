use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_vnode::{Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- LABEL ---

pub struct LabelLogic {
    pub text: String,
}

pub struct LabelView {
    pub core: ViewCore,
}

pub struct Label {
    pub id: String,
    pub logic: LabelLogic,
    pub view: LabelView,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: LabelLogic { text: text.into() },
            view: LabelView { core: view },
        }
    }
}

impl Component for Label {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "label".to_string(),
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

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let text_color = Color::Semantic("text".into(), None).to_rgba();
        renderer.draw_text(&self.logic.text, global_pos.x + layout.location.x, global_pos.y + layout.location.y, layout.size.width, 14.0, text_color, TextAlign::Left);
    }
}

impl Stylable for Label {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- INPUT ---

pub struct InputLogic {
    pub value: Signal<String>,
    pub placeholder: String,
}

pub struct InputView {
    pub core: ViewCore,
}

pub struct Input {
    pub id: String,
    pub logic: InputLogic,
    pub view: InputView,
}

impl Input {
    pub fn new(placeholder: impl Into<String>) -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: InputLogic {
                value: Signal::new(String::new()),
                placeholder: placeholder.into(),
            },
            view: InputView { core: view },
        }
    }
}

impl Component for Input {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "input".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("placeholder", self.logic.placeholder.clone());
                attr.insert("value", self.logic.value.get());
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

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.read().unwrap();
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

impl Stylable for Input {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- CHECKBOX ---

pub struct CheckboxLogic {
    pub checked: Signal<bool>,
}

pub struct CheckboxView {
    pub core: ViewCore,
}

pub struct Checkbox {
    pub id: String,
    pub logic: CheckboxLogic,
    pub view: CheckboxView,
}

impl Checkbox {
    pub fn new() -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: CheckboxLogic { checked: Signal::new(false) },
            view: CheckboxView { core: view },
        }
    }
}

impl Component for Checkbox {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "checkbox".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("checked", self.logic.checked.get().to_string());
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

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.read().unwrap();
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
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- SWITCH ---

pub struct SwitchLogic {
    pub active: Signal<bool>,
}

pub struct SwitchView {
    pub core: ViewCore,
}

pub struct Switch {
    pub id: String,
    pub logic: SwitchLogic,
    pub view: SwitchView,
}

impl Switch {
    pub fn new() -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: SwitchLogic { active: Signal::new(false) },
            view: SwitchView { core: view },
        }
    }
}

impl Component for Switch {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "switch".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("active", self.logic.active.get().to_string());
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

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.read().unwrap();
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
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- RADIO ---

pub struct RadioLogic {
    pub selected: Signal<bool>,
}

pub struct RadioView {
    pub core: ViewCore,
}

pub struct Radio {
    pub id: String,
    pub logic: RadioLogic,
    pub view: RadioView,
}

impl Radio {
    pub fn new() -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: RadioLogic { selected: Signal::new(false) },
            view: RadioView { core: view },
        }
    }
}

impl Component for Radio {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "radio".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("selected", self.logic.selected.get().to_string());
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

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.read().unwrap();
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
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- SELECT ---

pub struct SelectLogic {
    pub options: Vec<String>,
    pub selected_index: Signal<Option<usize>>,
}

pub struct SelectView {
    pub core: ViewCore,
}

pub struct Select {
    pub id: String,
    pub logic: SelectLogic,
    pub view: SelectView,
}

impl Select {
    pub fn new() -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: SelectLogic {
                options: vec![],
                selected_index: Signal::new(None),
            },
            view: SelectView { core: view },
        }
    }
}

impl Component for Select {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "select".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("options_count", self.logic.options.len().to_string());
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

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.read().unwrap();
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
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
