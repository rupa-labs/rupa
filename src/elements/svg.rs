use crate::support::{Style, generate_id, StyleModifier, Vec2, Attributes, Accessibility, Theme, Color, ColorMode, Role};
use crate::core::component::Component;
use crate::core::ViewCore;
use crate::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use crate::scene::SceneNode;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- ICON ---

pub struct IconLogic {
    pub name: String,
    pub size: f32,
    pub color: [f32; 4],
}

pub struct IconView { pub core: ViewCore }

pub struct Icon {
    pub id: String,
    pub logic: IconLogic,
    pub view: IconView,
}

impl Icon {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            logic: IconLogic { name: name.into(), size: 24.0, color: [1.0, 1.0, 1.0, 1.0] },
            view: IconView { core: ViewCore::default() },
        }
    }
    pub fn size(mut self, s: f32) -> Self { self.logic.size = s; self.view.core.mark_dirty(); self }
    pub fn color(mut self, c: [f32; 4]) -> Self { self.logic.color = c; self }
}

impl Component for Icon {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let mut s = taffy::style::Style::default();
        s.size = Size { width: length(self.logic.size), height: length(self.logic.size) };
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), s).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(s).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        renderer.draw_rect(global_pos.x, global_pos.y, self.logic.size, self.logic.size, self.logic.color, 2.0);
    }
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}

// --- SVG ---

pub struct Path { pub points: Vec<Vec2>, pub color: [f32; 4], pub thickness: f32 }
pub struct SvgLogic { pub paths: Vec<Path>, pub attributes: Attributes, pub accessibility: Accessibility }
pub struct SvgView { pub core: ViewCore }
pub struct Svg { pub id: String, pub logic: SvgLogic, pub view: SvgView }

impl Svg {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            logic: SvgLogic { paths: Vec::new(), attributes: Attributes::default(), accessibility: Accessibility::default() },
            view: SvgView { core: ViewCore::default() },
        }
    }
    pub fn add_path(mut self, path: Path) -> Self { self.logic.paths.push(path); self }
}

impl Stylable for Svg { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }

impl Component for Svg {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
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
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.borrow();
        let style = if is_group_hovered && style_ref.group_hover.is_some() { style_ref.group_hover.as_ref().unwrap() } else { &style_ref };
        if let Some(color) = style.background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), 0.0); }
        for path in &self.logic.paths { for point in &path.points { renderer.draw_rect(global_pos.x + point.x, global_pos.y + point.y, path.thickness, path.thickness, path.color, 0.0); } }
    }
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}

// --- THEME SWITCHER ---

pub struct ThemeSwitcherLogic { pub accessibility: Accessibility }
pub struct ThemeSwitcherView { pub core: ViewCore }
pub struct ThemeSwitcher { pub id: String, pub logic: ThemeSwitcherLogic, pub view: ThemeSwitcherView }

impl ThemeSwitcher {
    pub fn new() -> Self {
        Self { 
            id: generate_id(), 
            logic: ThemeSwitcherLogic { accessibility: Accessibility { role: Role::Button, ..Default::default() } },
            view: ThemeSwitcherView { core: ViewCore::default() },
        }
    }
    pub fn cycle_mode() {
        Theme::update(|t| {
            t.mode = match t.mode { ColorMode::Dark => ColorMode::Light, ColorMode::Light => ColorMode::Dark, ColorMode::System => ColorMode::Dark };
        });
    }
}

impl Stylable for ThemeSwitcher { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }

impl Component for ThemeSwitcher {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
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
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let color = if Theme::current().mode == ColorMode::Dark { [0.2, 0.2, 0.2, 1.0] } else { [0.9, 0.9, 0.9, 1.0] };
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color, 4.0);
    }
    fn on_click(&self, _event: &mut UIEvent) { Self::cycle_mode(); }
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}
