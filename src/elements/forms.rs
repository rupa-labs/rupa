use crate::utils::{Style, generate_id, Signal, Theme, Color, Accessibility, Attributes, Vec2, TextAlign, Role};
use crate::core::component::Component;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- LABEL ---

pub struct LabelLogic {
    pub text: String,
    pub accessibility: Accessibility,
}

pub struct LabelView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Label {
    pub id: String,
    pub logic: LabelLogic,
    pub view: LabelView,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            logic: LabelLogic {
                text: text.into(),
                accessibility: Accessibility { role: Role::Status, ..Default::default() },
            },
            view: LabelView {
                style: RefCell::new(Style::default()),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
}

impl Stylable for Label {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl Component for Label {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, self.view.style.borrow().to_taffy()).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_leaf(self.view.style.borrow().to_taffy()).unwrap();
            self.set_node(new_node);
            new_node
        };
        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        renderer.draw_text(&self.logic.text, global_pos.x, global_pos.y, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}

// --- CHECKBOX ---

pub struct CheckboxLogic {
    pub checked: Signal<bool>,
}

pub struct CheckboxView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Checkbox {
    pub id: String,
    pub logic: CheckboxLogic,
    pub view: CheckboxView,
}

impl Checkbox {
    pub fn new(checked: Signal<bool>) -> Self {
        Self {
            id: generate_id(),
            logic: CheckboxLogic { checked },
            view: CheckboxView {
                style: RefCell::new(Style::default()),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
}

impl Stylable for Checkbox {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl Component for Checkbox {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, self.view.style.borrow().to_taffy()).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_leaf(self.view.style.borrow().to_taffy()).unwrap();
            self.set_node(new_node);
            new_node
        };
        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let color = if self.logic.checked.get() { [0.0, 0.5, 1.0, 1.0] } else { [0.2, 0.2, 0.2, 1.0] };
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color, 4.0);
    }
    
    fn on_click(&self, _event: &mut UIEvent) { self.logic.checked.update(|v| *v = !*v); }
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}

// --- RADIO ---

pub struct RadioLogic {
    pub selected: Signal<bool>,
}

pub struct RadioView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Radio {
    pub id: String,
    pub logic: RadioLogic,
    pub view: RadioView,
}

impl Radio {
    pub fn new(selected: Signal<bool>) -> Self {
        Self {
            id: generate_id(),
            logic: RadioLogic { selected },
            view: RadioView {
                style: RefCell::new(Style::default()),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
}

impl Stylable for Radio {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl Component for Radio {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, self.view.style.borrow().to_taffy()).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_leaf(self.view.style.borrow().to_taffy()).unwrap();
            self.set_node(new_node);
            new_node
        };
        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let color = if self.logic.selected.get() { [0.0, 0.5, 1.0, 1.0] } else { [0.2, 0.2, 0.2, 1.0] };
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color, layout.size.width / 2.0);
    }

    fn on_click(&self, _event: &mut UIEvent) { self.logic.selected.set(true); }
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}

// --- SWITCH ---

pub struct SwitchLogic {
    pub on: Signal<bool>,
}

pub struct SwitchView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Switch {
    pub id: String,
    pub logic: SwitchLogic,
    pub view: SwitchView,
}

impl Switch {
    pub fn new(on: Signal<bool>) -> Self {
        Self {
            id: generate_id(),
            logic: SwitchLogic { on },
            view: SwitchView {
                style: RefCell::new(Style::default()),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
}

impl Stylable for Switch {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl Component for Switch {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, self.view.style.borrow().to_taffy()).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_leaf(self.view.style.borrow().to_taffy()).unwrap();
            self.set_node(new_node);
            new_node
        };
        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let color = if self.logic.on.get() { [0.0, 0.8, 0.4, 1.0] } else { [0.3, 0.3, 0.3, 1.0] };
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color, layout.size.height / 2.0);
    }

    fn on_click(&self, _event: &mut UIEvent) { self.logic.on.update(|v| *v = !*v); }
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}

// --- INPUT ---

pub struct InputLogic {
    pub value: Signal<String>,
    pub placeholder: String,
}

pub struct InputView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Input {
    pub id: String,
    pub logic: InputLogic,
    pub view: InputView,
}

impl Input {
    pub fn new(value: Signal<String>, placeholder: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            logic: InputLogic { value, placeholder: placeholder.into() },
            view: InputView {
                style: RefCell::new(Style::default()),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
}

impl Stylable for Input {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl Component for Input {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, self.view.style.borrow().to_taffy()).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_leaf(self.view.style.borrow().to_taffy()).unwrap();
            self.set_node(new_node);
            new_node
        };
        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.1, 0.1, 0.1, 1.0], 4.0);
        let text = if self.logic.value.get().is_empty() { &self.logic.placeholder } else { &self.logic.value.get() };
        renderer.draw_text(text, global_pos.x + 8.0, global_pos.y + 4.0, 14.0, [0.8, 0.8, 0.8, 1.0], TextAlign::Left);
    }

    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}

// --- SELECT ---

pub struct SelectLogic {
    pub options: Vec<String>,
    pub selected_index: Signal<usize>,
}

pub struct SelectView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Select {
    pub id: String,
    pub logic: SelectLogic,
    pub view: SelectView,
}

impl Select {
    pub fn new(options: Vec<String>, selected: Signal<usize>) -> Self {
        Self {
            id: generate_id(),
            logic: SelectLogic { options, selected_index: selected },
            view: SelectView {
                style: RefCell::new(Style::default()),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
}

impl Stylable for Select {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl Component for Select {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, self.view.style.borrow().to_taffy()).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_leaf(self.view.style.borrow().to_taffy()).unwrap();
            self.set_node(new_node);
            new_node
        };
        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.15, 0.15, 0.15, 1.0], 4.0);
        if let Some(opt) = self.logic.options.get(self.logic.selected_index.get()) {
            renderer.draw_text(opt, global_pos.x + 8.0, global_pos.y + 4.0, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
        }
    }

    fn on_click(&self, _event: &mut UIEvent) {
        let len = self.logic.options.len();
        if len > 0 {
            self.logic.selected_index.update(|i| *i = (*i + 1) % len);
        }
    }
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}
