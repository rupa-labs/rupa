use crate::elements::children::Children;
use rupa_core::{Component, VNode, Id, Signal};
use rupa_vnode::{Style, Theme, Variant};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ButtonSize { Xs, Sm, #[default] Md, Lg, Xl, Xl2, Xl3, Xl4, Xl5, Xl6 }

/// A standard interactive button.
pub struct Button<'a> {
    pub id: String,
    pub label: String,
    pub variant: Variant,
    pub size: ButtonSize,
    pub disabled: Signal<bool>,
    pub on_click: Option<Arc<dyn Fn(rupa_core::events::UIEvent) + Send + Sync>>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
    pub children: Children<'a>,
}

impl<'a> Button<'a> {
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        let mut style = Style::default();
        let variant = Variant::Primary;
        style.background.color = Some(Theme::variant(variant.clone()));

        Self {
            id: Id::next().to_string(),
            label,
            variant,
            size: ButtonSize::Md,
            disabled: Signal::new(false),
            on_click: None,
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
            children: Children::new(),
        }
    }

    pub fn variant(self, v: Variant) -> Self { 
        self.get_style().write().unwrap().background.color = Some(Theme::variant(v.clone()));
        Self { variant: v, ..self }
    }

    pub fn on_click(mut self, f: impl Fn(rupa_core::events::UIEvent) + Send + Sync + 'static) -> Self { 
        self.on_click = Some(Arc::new(f)); 
        self 
    }

    pub fn disabled(self, disabled: bool) -> Self {
        self.disabled.set(disabled);
        self
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.id = key.into();
        self
    }
}

impl<'a> Stylable for Button<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.get_style().write().unwrap() }
}

impl<'a> Component for Button<'a> {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        let is_disabled = self.disabled.get();
        let mut node = VNode::element("button")
            .with_style(self.get_style().read().unwrap().clone())
            .with_attr("disabled", if is_disabled { "true" } else { "false" })
            .with_children(self.children.render_all())
            .with_child(VNode::text(self.label.clone()))
            .with_key(self.id.clone());

        if !is_disabled {
            if let Some(ref handler) = self.on_click {
                node = node.with_arc_handler(handler.clone());
            }
        }

        node
    }
}

/// A specialized red button for destructive actions.
pub struct CloseButton {
    pub id: String,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl CloseButton {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.background.color = Some(rupa_vnode::Color::Rgba(1.0, 0.0, 0.0, 1.0));
        Self { 
            id: Id::next().to_string(), 
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Stylable for CloseButton { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.get_style().write().unwrap() } }

impl Component for CloseButton {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn render(&self) -> VNode {
        VNode::element("button")
            .with_style(self.get_style().read().unwrap().clone())
            .with_child(VNode::text("×"))
            .with_key(self.id.clone())
    }
}

/// A container for grouping buttons together.
pub struct ButtonGroup<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl<'a> ButtonGroup<'a> {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.flex.flex_direction = rupa_vnode::FlexDirection::Row;
        Self { 
            id: Id::next().to_string(), 
            children: Children::new(), 
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
    
    pub fn child(mut self, child: impl Component + 'a) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl<'a> Stylable for ButtonGroup<'a> { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.get_style().write().unwrap() } }

impl<'a> Component for ButtonGroup<'a> {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    fn render(&self) -> VNode {
        VNode::element("div")
            .with_style(self.get_style().read().unwrap().clone())
            .with_children(self.children.render_all())
            .with_key(self.id.clone())
    }
}
