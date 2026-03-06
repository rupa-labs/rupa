use crate::utils::{Style, generate_id, Accessibility, Role, Signal, Variant, Attributes, EventListeners, Theme, Color, Scale, TextAlign, Vec2, Display, FlexDirection, Spacing, Rounding};
use crate::core::component::Component;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use std::sync::Arc;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- LOGIC (The Brain) ---
// Pure UI logic and state. No rendering or layout dependencies here.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum ButtonSize { Xs, Sm, #[default] Md, Lg, Xl, Xl2, Xl3, Xl4, Xl5, Xl6 }

impl From<Scale> for ButtonSize {
    fn from(s: Scale) -> Self {
        match s {
            Scale::Xs => ButtonSize::Xs, Scale::Sm => ButtonSize::Sm, Scale::Md => ButtonSize::Md,
            Scale::Lg => ButtonSize::Lg, Scale::Xl => ButtonSize::Xl, Scale::Xl2 => ButtonSize::Xl2,
            Scale::Xl3 => ButtonSize::Xl3, Scale::Xl4 => ButtonSize::Xl4, Scale::Xl5 => ButtonSize::Xl5,
            Scale::Xl6 => ButtonSize::Xl6,
        }
    }
}

pub struct ButtonLogic {
    pub label: String,
    pub variant: Variant,
    pub size: ButtonSize,
    pub disabled: Signal<bool>,
    pub is_loading: Signal<bool>,
    pub events: EventListeners,
    pub accessibility: Accessibility,
    pub attributes: Attributes,
}

impl ButtonLogic {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: Variant::Primary,
            size: ButtonSize::Md,
            disabled: Signal::new(false),
            is_loading: Signal::new(false),
            events: EventListeners::default(),
            accessibility: Accessibility { role: Role::Button, ..Default::default() },
            attributes: Attributes::new(),
        }
    }

    pub fn handle_click(&self, event: &mut UIEvent) {
        if !self.disabled.get() && !self.is_loading.get() {
            if let Some(ref cb) = self.events.on_click {
                (cb)(event);
            }
        }
    }
}

// --- VIEW (The Body) ---
// Pure visual and infrastructure detail. Manages Taffy nodes and WGPU calls.
pub struct ButtonView {
    pub style: RefCell<Style>,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

impl ButtonView {
    pub fn new() -> Self {
        let theme = Theme::current();
        let mut style = Style::default();
        theme.apply_defaults(&mut style);
        
        Self {
            style: RefCell::new(style),
            node: Cell::new(None),
            dirty: AtomicBool::new(true),
        }
    }

    pub fn mark_dirty(&self) {
        self.dirty.store(true, Ordering::Relaxed);
    }

    pub fn compute_layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>, logic: &ButtonLogic) -> NodeId {
        let mut t_style = self.style.borrow().to_taffy();
        t_style.display = taffy::Display::Flex;
        t_style.flex_direction = taffy::FlexDirection::Row;
        t_style.align_items = Some(taffy::AlignItems::Center);
        
        match logic.size {
            ButtonSize::Xs => t_style.padding = Spacing::all(4.0).to_taffy(),
            ButtonSize::Sm => t_style.padding = Spacing::all(6.0).to_taffy(),
            ButtonSize::Md => t_style.padding = Spacing::all(8.0).to_taffy(),
            ButtonSize::Lg => t_style.padding = Spacing::all(12.0).to_taffy(),
            ButtonSize::Xl => t_style.padding = Spacing::all(16.0).to_taffy(),
            _ => t_style.padding = Spacing::all(20.0).to_taffy(),
        }

        let node = if let Some(existing) = self.node.get() {
            if self.dirty.load(Ordering::Relaxed) {
                taffy.set_style(existing, t_style).unwrap();
            }
            existing
        } else {
            let new_node = taffy.new_leaf(t_style).unwrap();
            self.node.set(Some(new_node));
            new_node
        };

        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) {
                taffy.add_child(p, node).unwrap();
            }
        }

        self.dirty.store(false, Ordering::Relaxed);
        node
    }

    pub fn render(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, logic: &ButtonLogic, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.style.borrow();
        
        let style: &Style = if is_group_hovered && style_ref.group_hover.is_some() { 
            style_ref.group_hover.as_ref().unwrap() 
        } else { 
            &style_ref 
        };
        
        if let Some(color) = style.background.color.clone() { 
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw); 
        }

        let text_color = if logic.disabled.get() { [0.5, 0.5, 0.5, 1.0] } else { [1.0, 1.0, 1.0, 1.0] };
        renderer.draw_text(&logic.label, global_pos.x + 8.0, global_pos.y + 4.0, 14.0, text_color, TextAlign::Left);
    }
}

// --- PUBLIC COMPONENT (The Bridge) ---
// High-level wrapper that coordinates Logic and View.
pub struct Button {
    pub id: String,
    pub logic: ButtonLogic,
    pub view: ButtonView,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        let logic = ButtonLogic::new(label);
        let view = ButtonView::new();
        
        let variant_color = Theme::variant(logic.variant.clone());
        view.style.borrow_mut().background.color = Some(variant_color);

        Self {
            id: generate_id(),
            logic,
            view,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    
    pub fn variant(self, v: Variant) -> Self { 
        self.view.style.borrow_mut().background.color = Some(Theme::variant(v.clone()));
        self.view.mark_dirty();
        self 
    }

    pub fn size(mut self, s: impl Into<ButtonSize>) -> Self { 
        self.logic.size = s.into(); 
        self.view.mark_dirty(); 
        self 
    }

    pub fn on_click(mut self, f: impl Fn(&mut UIEvent) + Send + Sync + 'static) -> Self { 
        self.logic.events.on_click = Some(Arc::new(f)); 
        self 
    }
}

impl Stylable for Button {
    fn get_style_mut(&self) -> RefMut<'_, Style> {
        self.view.style.borrow_mut()
    }
}

impl Component for Button {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<NodeId> { self.view.node.get() }
    fn set_node(&self, node: NodeId) { self.view.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.view.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        self.view.compute_layout(taffy, parent, &self.logic)
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        self.view.render(renderer, taffy, node, is_group_hovered, &self.logic, global_pos);
    }

    fn on_click(&self, event: &mut UIEvent) {
        self.logic.handle_click(event);
    }
    
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}

pub struct ButtonGroup<'a> { 
    pub id: String, 
    pub style: RefCell<Style>, 
    pub attributes: Attributes, 
    pub children: crate::elements::layout::container::Children<'a>, 
    pub accessibility: Accessibility,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

impl<'a> ButtonGroup<'a> {
    pub fn new() -> Self {
        let mut style = Style::default(); 
        style.layout.display = Display::Flex;
        Self { 
            id: generate_id(), 
            style: RefCell::new(style), 
            attributes: Attributes::new(), 
            children: crate::elements::layout::container::Children::new(), 
            accessibility: Accessibility { role: Role::Status, ..Default::default() },
            node: Cell::new(None),
            dirty: AtomicBool::new(true),
        }
    }
    pub fn child(mut self, button: Button) -> Self { self.children.add(Box::new(button)); self.mark_dirty(); self }
}

impl<'a> Stylable for ButtonGroup<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> {
        self.style.borrow_mut()
    }
}

impl<'a> Component for ButtonGroup<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.children.get_all() }
    fn get_node(&self) -> Option<NodeId> { self.node.get() }
    fn set_node(&self, node: NodeId) { self.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.dirty.store(false, Ordering::Relaxed); }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() { taffy.set_style(existing, self.style.borrow().to_taffy()).unwrap(); }
            existing
        } else {
            let new_node = taffy.new_with_children(self.style.borrow().to_taffy(), &[]).unwrap();
            self.set_node(new_node);
            new_node
        };
        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.children.layout_all(taffy, node);
        self.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        self.children.paint_all(renderer, taffy, node, is_group_hovered || self.style.borrow().is_group, render_pass, global_pos, 0);
    }
    fn on_click(&self, _event: &mut UIEvent) {}
    fn on_scroll(&self, event: &mut UIEvent, delta: f32) { self.children.list.iter().for_each(|c| c.on_scroll(event, delta)); }
    fn on_drag(&self, event: &mut UIEvent, delta: Vec2) { self.children.list.iter().for_each(|c| c.on_drag(event, delta)); }
}
