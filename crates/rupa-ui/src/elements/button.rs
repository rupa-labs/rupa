use crate::Children;
use rupa_core::{Component, VNode, Vec2, ViewCore, generate_id, Signal, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners};
use rupa_vnode::{Style, Color, Theme, Variant, Scale, Accessibility, TextAlign, Attributes};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

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

/// A standard interactive button.
pub struct Button {
    pub id: String,
    pub label: String,
    pub variant: Variant,
    pub size: ButtonSize,
    pub disabled: Signal<bool>,
    pub events: EventListeners,
    pub accessibility: Accessibility,
    pub view: Arc<ViewCore>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        let view = Arc::new(ViewCore::new());
        let variant = Variant::Primary;
        
        // Apply initial theme color
        view.style().background.color = Some(Theme::variant(variant.clone()));

        Self {
            id: generate_id(),
            label,
            variant,
            size: ButtonSize::Md,
            disabled: Signal::new(false),
            events: EventListeners::default(),
            accessibility: Accessibility::default(),
            view,
        }
    }

    pub fn variant(self, v: Variant) -> Self { 
        self.view.style().background.color = Some(Theme::variant(v.clone()));
        self.view.mark_dirty();
        Self { variant: v, ..self }
    }

    pub fn on_click(mut self, f: impl Fn(&mut UIEvent) + Send + Sync + 'static) -> Self { 
        self.events.on_click = Some(Arc::new(f)); 
        self 
    }

    pub fn disabled(self, disabled: bool) -> Self {
        self.disabled.set(disabled);
        self
    }
}

impl Stylable for Button {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

impl Component for Button {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("button")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
            .with_child(VNode::text(self.label.clone()))
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let mut style = self.view.style().to_taffy();
        style.display = taffy::Display::Flex;
        
        // Size-based padding fallback if not set manually
        if self.view.style.read().unwrap().padding.is_zero() {
            let p = match self.size {
                ButtonSize::Xs => 4.0, ButtonSize::Sm => 6.0, ButtonSize::Md => 8.0, _ => 12.0,
            };
            style.padding = rupa_vnode::Spacing::all(p).to_taffy();
        }

        let node = taffy.new_leaf(style).unwrap();
        self.view.set_node(SceneNode::from(node));
        self.view.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style = self.view.style.read().unwrap();
        
        if let Some(color) = style.background.color.clone() { 
            renderer.draw_rect(
                global_pos.x + layout.location.x, 
                global_pos.y + layout.location.y, 
                layout.size.width, 
                layout.size.height, 
                color.to_rgba(), 
                style.rounding.nw
            ); 
        }

        let text_color = if self.disabled.get() { [0.5, 0.5, 0.5, 1.0] } else { [1.0, 1.0, 1.0, 1.0] };
        // Center text horizontally and vertically roughly
        renderer.draw_text(&self.label, global_pos.x + layout.location.x + 8.0, global_pos.y + layout.location.y + 4.0, layout.size.width - 16.0, 14.0, text_color, TextAlign::Left);
    }

    fn on_click(&self, event: &mut UIEvent) {
        if !self.disabled.get() {
            if let Some(ref cb) = self.events.on_click { (cb)(event); }
        }
    }

    fn on_release(&self, event: &mut UIEvent) {
        if let Some(ref cb) = self.events.on_release { (cb)(event); }
    }
}

/// A specialized red button for destructive actions.
pub struct CloseButton {
    pub id: String,
    pub view: Arc<ViewCore>,
}

impl CloseButton {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        view.style().background.color = Some(Color::Rgba(1.0, 0.0, 0.0, 1.0));
        Self { id: generate_id(), view }
    }
}

impl Stylable for CloseButton { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() } }

impl Component for CloseButton {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    fn render(&self) -> VNode {
        VNode::element("button")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
            .with_child(VNode::text("×"))
    }
    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.view.style().to_taffy()).unwrap();
        self.view.set_node(SceneNode::from(node));
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style = self.view.style.read().unwrap();
        if let Some(color) = style.background.color.clone() { 
            renderer.draw_rect(global_pos.x + layout.location.x, global_pos.y + layout.location.y, layout.size.width, layout.size.height, color.to_rgba(), 4.0);
        }
        renderer.draw_text("×", global_pos.x + layout.location.x + 4.0, global_pos.y + layout.location.y + 2.0, layout.size.width, 14.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
}

/// A container for grouping buttons together.
pub struct ButtonGroup<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub view: Arc<ViewCore>,
}

impl<'a> ButtonGroup<'a> {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        view.style().flex.flex_direction = rupa_vnode::FlexDirection::Row;
        Self { id: generate_id(), children: Children::new(), view }
    }
    pub fn child(mut self, child: Button) -> Self {
        self.children.push(Box::new(child));
        self.view.mark_dirty();
        self
    }
}

impl<'a> Stylable for ButtonGroup<'a> { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() } }

impl<'a> Component for ButtonGroup<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    fn render(&self) -> VNode {
        VNode::Element(rupa_vnode::VElement {
            tag: "button-group".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: self.children.render_all(),
            key: Some(self.id.clone()),
        })
    }
    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.view.style().to_taffy(), &[]).unwrap();
        self.view.set_node(SceneNode::from(node));
        let child_nodes = self.children.layout_all(taffy, measurer);
        taffy.set_children(node, &child_nodes).unwrap();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let style_ref = self.view.style.read().unwrap();
        self.children.paint_all(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos, 0);
    }
}
