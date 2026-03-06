use crate::support::{Style, generate_id, Signal, Theme, Color, Accessibility, Attributes, Vec2, TextAlign, Role};
use crate::core::component::Component;
use crate::core::ViewCore;
use crate::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use crate::scene::SceneNode;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- LABEL ---
pub struct LabelLogic { pub text: String, pub accessibility: Accessibility }
pub struct LabelView { pub core: ViewCore }
pub struct Label { pub id: String, pub logic: LabelLogic, pub view: LabelView }
impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self { id: generate_id(), logic: LabelLogic { text: text.into(), accessibility: Accessibility { role: Role::Status, ..Default::default() } }, view: LabelView { core: ViewCore::default() } }
    }
}
impl Stylable for Label { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Label {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, _: &TaffyTree<()>, _: NodeId, _: bool, global_pos: Vec2) {
        renderer.draw_text(&self.logic.text, global_pos.x, global_pos.y, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}

// --- CHECKBOX ---
pub struct CheckboxLogic { pub checked: Signal<bool> }
pub struct CheckboxView { pub core: ViewCore }
pub struct Checkbox { pub id: String, pub logic: CheckboxLogic, pub view: CheckboxView }
impl Checkbox {
    pub fn new(checked: Signal<bool>) -> Self {
        Self { id: generate_id(), logic: CheckboxLogic { checked }, view: CheckboxView { core: ViewCore::default() } }
    }
}
impl Stylable for Checkbox { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Checkbox {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let color = if self.logic.checked.get() { [0.0, 0.5, 1.0, 1.0] } else { [0.2, 0.2, 0.2, 1.0] };
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color, 4.0);
    }
    fn on_click(&self, _: &mut UIEvent) { self.logic.checked.update(|v| *v = !*v); }
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}

// --- RADIO ---
pub struct RadioLogic { pub selected: Signal<bool> }
pub struct RadioView { pub core: ViewCore }
pub struct Radio { pub id: String, pub logic: RadioLogic, pub view: RadioView }
impl Radio {
    pub fn new(selected: Signal<bool>) -> Self {
        Self { id: generate_id(), logic: RadioLogic { selected }, view: RadioView { core: ViewCore::default() } }
    }
}
impl Stylable for Radio { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Radio {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let color = if self.logic.selected.get() { [0.0, 0.5, 1.0, 1.0] } else { [0.2, 0.2, 0.2, 1.0] };
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color, layout.size.width / 2.0);
    }
    fn on_click(&self, _: &mut UIEvent) { self.logic.selected.set(true); }
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}

// --- SWITCH ---
pub struct SwitchLogic { pub on: Signal<bool> }
pub struct SwitchView { pub core: ViewCore }
pub struct Switch { pub id: String, pub logic: SwitchLogic, pub view: SwitchView }
impl Switch {
    pub fn new(on: Signal<bool>) -> Self {
        Self { id: generate_id(), logic: SwitchLogic { on }, view: SwitchView { core: ViewCore::default() } }
    }
}
impl Stylable for Switch { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Switch {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let color = if self.logic.on.get() { [0.0, 0.8, 0.4, 1.0] } else { [0.3, 0.3, 0.3, 1.0] };
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color, layout.size.height / 2.0);
    }
    fn on_click(&self, _: &mut UIEvent) { self.logic.on.update(|v| *v = !*v); }
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}

// --- INPUT ---
pub struct InputLogic { pub value: Signal<String>, pub placeholder: String }
pub struct InputView { pub core: ViewCore }
pub struct Input { pub id: String, pub logic: InputLogic, pub view: InputView }
impl Input {
    pub fn new(value: Signal<String>, placeholder: impl Into<String>) -> Self {
        Self { id: generate_id(), logic: InputLogic { value, placeholder: placeholder.into() }, view: InputView { core: ViewCore::default() } }
    }
}
impl Stylable for Input { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Input {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.1, 0.1, 0.1, 1.0], 4.0);
        let text = if self.logic.value.get().is_empty() { &self.logic.placeholder } else { &self.logic.value.get() };
        renderer.draw_text(text, global_pos.x + 8.0, global_pos.y + 4.0, 14.0, [0.8, 0.8, 0.8, 1.0], TextAlign::Left);
    }
    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}

// --- SELECT ---
pub struct SelectLogic { pub options: Vec<String>, pub selected_index: Signal<usize> }
pub struct SelectView { pub core: ViewCore }
pub struct Select { pub id: String, pub logic: SelectLogic, pub view: SelectView }
impl Select {
    pub fn new(options: Vec<String>, selected: Signal<usize>) -> Self {
        Self { id: generate_id(), logic: SelectLogic { options, selected_index: selected }, view: SelectView { core: ViewCore::default() } }
    }
}
impl Stylable for Select { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Select {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.15, 0.15, 0.15, 1.0], 4.0);
        if let Some(opt) = self.logic.options.get(self.logic.selected_index.get()) {
            renderer.draw_text(opt, global_pos.x + 8.0, global_pos.y + 4.0, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
        }
    }
    fn on_click(&self, _: &mut UIEvent) { let len = self.logic.options.len(); if len > 0 { self.logic.selected_index.update(|i| *i = (*i + 1) % len); } }
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}
