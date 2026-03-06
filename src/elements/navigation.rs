use crate::utils::{Style, generate_id, Theme, Color, Accessibility, Attributes, Signal, TextAlign, Vec2, Display, FlexDirection, JustifyContent};
use crate::core::component::Component;
use crate::elements::layout::container::Children;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- NAVBAR ---

pub struct NavbarLogic<'a> {
    pub start: Children<'a>,
    pub center: Children<'a>,
    pub end: Children<'a>,
    pub accessibility: Accessibility,
}

pub struct NavbarView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Navbar<'a> {
    pub id: String,
    pub logic: NavbarLogic<'a>,
    pub view: NavbarView,
}

impl<'a> Navbar<'a> {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        style.layout.display = Display::Flex;
        style.flex.flex_direction = FlexDirection::Row;
        style.padding = crate::utils::Spacing::all(16.0);
        style.background.color = Some(Color::Semantic("surface".into(), None));

        Self {
            id: generate_id(),
            logic: NavbarLogic {
                start: Children::new(),
                center: Children::new(),
                end: Children::new(),
                accessibility: Accessibility { role: crate::utils::Role::Navigation, ..Default::default() },
            },
            view: NavbarView {
                style: RefCell::new(style),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }

    pub fn start(mut self, c: Box<dyn Component + 'a>) -> Self { self.logic.start.add(c); self.view.dirty.store(true, Ordering::Relaxed); self }
    pub fn center(mut self, c: Box<dyn Component + 'a>) -> Self { self.logic.center.add(c); self.view.dirty.store(true, Ordering::Relaxed); self }
    pub fn end(mut self, c: Box<dyn Component + 'a>) -> Self { self.logic.end.add(c); self.view.dirty.store(true, Ordering::Relaxed); self }
}

impl<'a> Stylable for Navbar<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl<'a> Component for Navbar<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> {
        let mut c = self.logic.start.get_all();
        c.extend(self.logic.center.get_all());
        c.extend(self.logic.end.get_all());
        c
    }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let mut t_style = self.view.style.borrow().to_taffy();
        t_style.justify_content = Some(JustifyContent::SpaceBetween);

        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, t_style).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_with_children(t_style, &[]).unwrap();
            self.set_node(new_node);
            new_node
        };

        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }

        let mut child_nodes = Vec::new();
        child_nodes.extend(self.logic.start.layout_all(taffy, node));
        child_nodes.extend(self.logic.center.layout_all(taffy, node));
        child_nodes.extend(self.logic.end.layout_all(taffy, node));
        taffy.set_children(node, &child_nodes).unwrap();

        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        if let Some(color) = self.view.style.borrow().background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), 0.0);
        }
        let mut idx = 0;
        self.logic.start.paint_all(renderer, taffy, node, is_group_hovered, render_pass, global_pos, idx);
        idx += self.logic.start.list.len();
        self.logic.center.paint_all(renderer, taffy, node, is_group_hovered, render_pass, global_pos, idx);
        idx += self.logic.center.list.len();
        self.logic.end.paint_all(renderer, taffy, node, is_group_hovered, render_pass, global_pos, idx);
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, event: &mut UIEvent, delta: f32) {
        self.logic.start.list.iter().for_each(|c| c.on_scroll(event, delta));
        self.logic.center.list.iter().for_each(|c| c.on_scroll(event, delta));
        self.logic.end.list.iter().for_each(|c| c.on_scroll(event, delta));
    }
    fn on_drag(&self, event: &mut UIEvent, delta: Vec2) {
        self.logic.start.list.iter().for_each(|c| c.on_drag(event, delta));
        self.logic.center.list.iter().for_each(|c| c.on_drag(event, delta));
        self.logic.end.list.iter().for_each(|c| c.on_drag(event, delta));
    }
}

// --- TABS ---

pub struct Tab<'a> { pub title: String, pub content: Box<dyn Component + 'a> }

pub struct TabsLogic<'a> {
    pub tabs: Vec<Tab<'a>>,
    pub active_index: Signal<usize>,
}

pub struct TabsView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Tabs<'a> {
    pub id: String,
    pub logic: TabsLogic<'a>,
    pub view: TabsView,
}

impl<'a> Tabs<'a> {
    pub fn new(tabs: Vec<Tab<'a>>, active: Signal<usize>) -> Self {
        Self {
            id: generate_id(),
            logic: TabsLogic { tabs, active_index: active },
            view: TabsView {
                style: RefCell::new(Style::default()),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
}

impl<'a> Stylable for Tabs<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl<'a> Component for Tabs<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> {
        if let Some(tab) = self.logic.tabs.get(self.logic.active_index.get()) {
            vec![tab.content.as_ref()]
        } else { vec![] }
    }
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
            let new_node = taffy.new_with_children(self.view.style.borrow().to_taffy(), &[]).unwrap();
            self.set_node(new_node);
            new_node
        };

        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }

        if let Some(tab) = self.logic.tabs.get(self.logic.active_index.get()) { 
            let child_node = tab.content.layout(taffy, Some(node));
            taffy.set_children(node, &[child_node]).unwrap();
        } else {
            taffy.set_children(node, &[]).unwrap();
        }
        self.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        if let Some(tab) = self.logic.tabs.get(self.logic.active_index.get()) {
            let children = taffy.children(node).unwrap();
            if let Some(c_node) = children.get(0) { 
                tab.content.paint(renderer, taffy, *c_node, is_group_hovered, render_pass, global_pos); 
            }
        }
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, event: &mut UIEvent, d: f32) { if let Some(tab) = self.logic.tabs.get(self.logic.active_index.get()) { tab.content.on_scroll(event, d); } }
    fn on_drag(&self, event: &mut UIEvent, d: Vec2) { if let Some(tab) = self.logic.tabs.get(self.logic.active_index.get()) { tab.content.on_drag(event, d); } }
}

// --- BREADCRUMB ---

pub struct BreadcrumbLogic {
    pub items: Vec<String>,
}

pub struct BreadcrumbView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

pub struct Breadcrumb {
    pub id: String,
    pub logic: BreadcrumbLogic,
    pub view: BreadcrumbView,
}

impl Breadcrumb {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            id: generate_id(),
            logic: BreadcrumbLogic { items },
            view: BreadcrumbView {
                style: RefCell::new(Style::default()),
                node: Cell::new(None),
                dirty: AtomicBool::new(true),
            },
        }
    }
}

impl Stylable for Breadcrumb {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.style.borrow_mut() }
}

impl Component for Breadcrumb {
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
        let text = self.logic.items.join(" / ");
        renderer.draw_text(&text, global_pos.x, global_pos.y, 12.0, [0.7, 0.7, 0.7, 1.0], TextAlign::Left);
    }
    
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}
