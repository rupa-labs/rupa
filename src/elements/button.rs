use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Signal, Variant, Attributes, EventListeners, Theme, Color, Scale, TextAlign, Vec2};
use crate::Component;
use crate::container::Children;
use crate::renderer::Renderer;
use std::sync::Arc;
use taffy::prelude::*;

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

pub struct Button<'a> {
    pub id: String, pub label: String, pub variant: Variant, pub size: ButtonSize, pub disabled: Signal<bool>, pub is_loading: Signal<bool>, pub style: Style, pub attributes: Attributes, pub events: EventListeners, pub accessibility: Accessibility,
    pub prefix: Option<Box<dyn Component + 'a>>, pub suffix: Option<Box<dyn Component + 'a>>,
}

impl<'a> Button<'a> {
    pub fn new(label: impl Into<String>) -> Self {
        let theme = Theme::current();
        let mut style = Style::default(); theme.apply_defaults(&mut style);
        let primary_color = theme.variants.get(&Variant::Primary).cloned().unwrap_or(Color::Semantic("primary".into(), None));
        style.background.color = Some(primary_color);
        Self { id: generate_id(), label: label.into(), variant: Variant::Primary, size: ButtonSize::Md, disabled: Signal::new(false), is_loading: Signal::new(false), style, attributes: Attributes::new(), events: EventListeners::default(), accessibility: Accessibility { role: Role::Button, ..Default::default() }, prefix: None, suffix: None }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn a11y(mut self, acc: Accessibility) -> Self { self.accessibility = acc; self }
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self { self.attributes.set(key, value); self }
    pub fn variant(mut self, v: Variant) -> Self { self.variant = v.clone(); self.style.background.color = Some(Theme::variant(v)); self }
    pub fn size(mut self, s: impl Into<ButtonSize>) -> Self {
        let s = s.into(); self.size = s.clone();
        match s {
            ButtonSize::Xs => self.style.padding = crate::utils::spacing::Spacing::all(4.0),
            ButtonSize::Sm => self.style.padding = crate::utils::spacing::Spacing::all(6.0),
            ButtonSize::Md => self.style.padding = crate::utils::spacing::Spacing::all(8.0),
            ButtonSize::Lg => self.style.padding = crate::utils::spacing::Spacing::all(12.0),
            ButtonSize::Xl => self.style.padding = crate::utils::spacing::Spacing::all(16.0),
            _ => self.style.padding = crate::utils::spacing::Spacing::all(20.0),
        }
        self
    }
    pub fn on_click(mut self, f: impl Fn() + Send + Sync + 'static) -> Self { self.events.on_click = Some(Arc::new(f)); self }
    pub fn trigger(&self) { if !self.disabled.get() { if let Some(ref cb) = self.events.on_click { (cb)(); } } }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
}

impl<'a> Component for Button<'a> {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let mut style = self.style.to_taffy(); style.display = Display::Flex; style.flex_direction = FlexDirection::Row; style.align_items = Some(AlignItems::Center);
        let node = taffy.new_leaf(style).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        if let Some(color) = style.background.color.clone() { 
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw); 
        }
        renderer.draw_text(&self.label, global_pos.x + 8.0, global_pos.y + 4.0, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
    fn on_click(&self) { self.trigger(); }
    fn on_scroll(&self, _: f32) {}
    fn on_drag(&self, _: Vec2) {}
}

pub struct CloseButton { pub id: String, pub disabled: Signal<bool>, pub style: Style, pub attributes: Attributes, pub accessibility: Accessibility }
impl CloseButton {
    pub fn new() -> Self { Self { id: generate_id(), disabled: Signal::new(false), style: Style::default(), attributes: Attributes::new(), accessibility: Accessibility { role: Role::Button, ..Default::default() } } }
}
impl Component for CloseButton {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.style.to_taffy()).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [1.0, 0.0, 0.0, 1.0], 0.0);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _: f32) {}
    fn on_drag(&self, _: Vec2) {}
}

pub struct ButtonGroup<'a> { pub id: String, pub style: Style, pub attributes: Attributes, pub children: Children<'a>, pub accessibility: Accessibility }
impl<'a> ButtonGroup<'a> {
    pub fn new() -> Self {
        let mut style = Style::default(); style.layout.display = crate::utils::Display::Flex;
        Self { id: generate_id(), style, attributes: Attributes::new(), children: Children::new(), accessibility: Accessibility { role: Role::Status, ..Default::default() } }
    }
    pub fn child(mut self, button: Button<'a>) -> Self { self.children.add(Box::new(button)); self }
}
impl<'a> Component for ButtonGroup<'a> {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        self.children.layout_all(taffy, node);
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        self.children.paint_all(renderer, taffy, node, is_group_hovered || self.style.is_group, render_pass, global_pos);
    }
    fn on_click(&self) {}
    fn on_scroll(&self, delta: f32) { self.children.list.iter().for_each(|c| c.on_scroll(delta)); }
    fn on_drag(&self, delta: Vec2) { self.children.list.iter().for_each(|c| c.on_drag(delta)); }
}
