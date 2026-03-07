use rupa_core::{Style, generate_id, Accessibility, Signal, Variant, EventListeners, Theme, Scale, Vec2, Spacing, Readable, Color};
use rupa_core::typography::TextAlign;
use rupa_core::component::Component;
use rupa_core::view::ViewCore;
use rupa_core::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::base::Stylable;
use rupa_core::events::UIEvent;
use rupa_core::scene::SceneNode;
use std::sync::Arc;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- LOGIC ---

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ButtonSize { Xs, Sm, #[default] Md, Lg, Xl, Xl2, Xl3, Xl4, Xl5, Xl6 }

impl From<Scale> for ButtonSize {
    fn from(s: Scale) -> Self {
        match s {
            Scale::Xs => ButtonSize::Xs, Scale::Sm => ButtonSize::Sm, Scale::Md => ButtonSize::Md,
            Scale::Lg => ButtonSize::Lg, Scale::Xl => ButtonSize::Xl, Scale::Xl2 => ButtonSize::Xl2,
            _ => ButtonSize::Md,
        }
    }
}

pub struct ButtonLogic {
    pub label: String,
    pub variant: Variant,
    pub size: ButtonSize,
    pub disabled: Signal<bool>,
    pub events: EventListeners,
    pub accessibility: Accessibility,
}

impl ButtonLogic {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: Variant::Primary,
            size: ButtonSize::Md,
            disabled: Signal::new(false),
            events: EventListeners::default(),
            accessibility: Accessibility::default(),
        }
    }
}

// --- VIEW ---

pub struct ButtonView {
    pub core: ViewCore,
}

impl ButtonView {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            core: ViewCore::new(style),
        }
    }

    pub fn compute_layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>, logic: &ButtonLogic) -> NodeId {
        let mut t_style = self.core.get_style_mut().to_taffy();
        t_style.display = taffy::Display::Flex;
        
        match logic.size {
            ButtonSize::Xs => t_style.padding = Spacing::all(4.0).to_taffy(),
            ButtonSize::Sm => t_style.padding = Spacing::all(6.0).to_taffy(),
            ButtonSize::Md => t_style.padding = Spacing::all(8.0).to_taffy(),
            _ => t_style.padding = Spacing::all(12.0).to_taffy(),
        }

        let node = if let Some(existing) = self.core.get_node() {
            if self.core.is_dirty() { taffy.set_style(existing.raw(), t_style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(t_style).unwrap();
            self.core.set_node(SceneNode::from(new_node));
            new_node
        };

        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }

        self.core.clear_dirty();
        node
    }

    pub fn render(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, logic: &ButtonLogic, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style = self.core.style.read().unwrap();
        
        if let Some(color) = style.background.color.clone() { 
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw); 
        }

        let text_color = if logic.disabled.get() { [0.5, 0.5, 0.5, 1.0] } else { [1.0, 1.0, 1.0, 1.0] };
        renderer.draw_text(&logic.label, global_pos.x + 8.0, global_pos.y + 4.0, layout.size.width - 16.0, 14.0, text_color, TextAlign::Left);
    }
}

// --- BRIDGE ---

pub struct Button {
    pub id: String,
    pub logic: ButtonLogic,
    pub view: ButtonView,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        let logic = ButtonLogic::new(label);
        let view = ButtonView::new();
        view.core.get_style_mut().background.color = Some(Theme::variant(logic.variant.clone()));

        Self {
            id: generate_id(),
            logic,
            view,
        }
    }

    pub fn variant(self, v: Variant) -> Self { 
        self.view.core.get_style_mut().background.color = Some(Theme::variant(v));
        self.view.core.mark_dirty();
        self 
    }

    pub fn on_click(mut self, f: impl Fn(&mut UIEvent) + Send + Sync + 'static) -> Self { 
        self.logic.events.on_click = Some(Arc::new(f)); 
        self 
    }
}

impl Stylable for Button {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() }
}

impl Component for Button {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        self.view.compute_layout(taffy, measurer, parent, &self.logic)
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        self.view.render(renderer, taffy, node, &self.logic, global_pos);
    }

    fn on_click(&self, event: &mut UIEvent) {
        if !self.logic.disabled.get() {
            if let Some(ref cb) = self.logic.events.on_click { (cb)(event); }
        }
    }
}

// --- CLOSE BUTTON ---

pub struct CloseButton { pub id: String, pub view: ButtonView }
impl CloseButton {
    pub fn new() -> Self {
        let view = ButtonView::new();
        view.core.get_style_mut().background.color = Some(Color::Rgba(1.0, 0.0, 0.0, 1.0));
        Self { id: generate_id(), view }
    }
}
impl Stylable for CloseButton { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() } }
impl Component for CloseButton {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        self.view.compute_layout(taffy, measurer, parent, &ButtonLogic::new("×"))
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        self.view.render(renderer, taffy, node, &ButtonLogic::new("×"), global_pos);
    }
}

// --- BUTTON GROUP ---

pub struct ButtonGroup<'a> { pub id: String, pub logic: crate::elements::layout::container::ContainerLogic<'a>, pub view: crate::elements::layout::container::ContainerView }
impl<'a> ButtonGroup<'a> {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.layout.display = crate::support::Display::Flex;
        style.flex.flex_direction = crate::support::FlexDirection::Row;
        Self { id: generate_id(), logic: crate::elements::layout::container::ContainerLogic { children: crate::elements::layout::container::Children::new() }, view: crate::elements::layout::container::ContainerView { core: ViewCore::new(style) } }
    }
    pub fn child(mut self, child: Button) -> Self { self.logic.children.add(Box::new(child)); self.view.core.mark_dirty(); self }
}
impl<'a> Stylable for ButtonGroup<'a> { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() } }
impl<'a> Component for ButtonGroup<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.get_all() }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(self.view.core.get_style_mut().to_taffy(), &[]).unwrap(); self.view.core.set_node(SceneNode::from(new_node)); new_node
        };
        let child_nodes = self.logic.children.layout_all(taffy, measurer);
        taffy.set_children(node, &child_nodes).unwrap();
        self.view.core.clear_dirty(); node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let style_ref = self.view.core.style.read().unwrap();
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos, 0);
    }
}
