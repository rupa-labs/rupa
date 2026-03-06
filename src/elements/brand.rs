use crate::support::{Style, StyleModifier, generate_id, Accessibility, Role, Attributes, Theme, TextAlign, Vec2};
use crate::core::component::Component;
use crate::core::ViewCore;
use crate::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use crate::scene::SceneNode;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- LOGIC ---

pub struct BrandLogic<'a> {
    pub name: String,
    pub logo_path: Option<String>,
    pub children: Vec<Box<dyn Component + 'a>>,
    pub accessibility: Accessibility,
}

// --- VIEW ---

pub struct BrandView {
    pub core: ViewCore,
}

impl BrandView {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self { core: ViewCore::new(style) }
    }

    pub fn compute_layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let t_style = self.core.get_style_mut().to_taffy();
        let node = if let Some(existing) = self.core.get_node() {
            if self.core.is_dirty() { taffy.set_style(existing.raw(), t_style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(t_style).unwrap();
            self.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent {
            let cur = taffy.children(p).unwrap_or_default();
            if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.core.clear_dirty();
        node
    }

    pub fn render(&self, renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, logic: &BrandLogic, global_pos: Vec2) {
        renderer.draw_text(&logic.name, global_pos.x, global_pos.y, 18.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
}

// --- COMPONENT ---

pub struct Brand<'a> {
    pub id: String,
    pub logic: BrandLogic<'a>,
    pub view: BrandView,
}

impl<'a> Brand<'a> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            logic: BrandLogic {
                name: name.into(),
                logo_path: None,
                children: Vec::new(),
                accessibility: Accessibility { role: Role::Navigation, ..Default::default() },
            },
            view: BrandView::new(),
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn logo(mut self, path: impl Into<String>) -> Self { 
        self.logic.logo_path = Some(path.into()); 
        self.view.core.mark_dirty(); 
        self 
    }
}

impl<'a> Stylable for Brand<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() }
}

impl<'a> Component for Brand<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.iter().map(|c| c.as_ref()).collect() }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        self.view.compute_layout(taffy, measurer, parent)
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        self.view.render(renderer, taffy, node, &self.logic, global_pos);
    }

    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}
