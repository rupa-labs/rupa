use crate::Children;
use rupa_core::{Component, VNode, ViewCore, Id, Signal};
use rupa_vnode::{Style, Theme, Variant, Accessibility, Attributes};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ButtonSize { Xs, Sm, #[default] Md, Lg, Xl, Xl2, Xl3, Xl4, Xl5, Xl6 }

/// A standard interactive button.
pub struct Button {
    pub id: String,
    pub label: String,
    pub variant: Variant,
    pub size: ButtonSize,
    pub disabled: Signal<bool>,
    pub on_click: Option<Arc<dyn Fn(rupa_vnode::UIEvent) + Send + Sync>>,
    pub accessibility: Accessibility,
    pub view: Arc<ViewCore>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        let view = Arc::new(ViewCore::new());
        let variant = Variant::Primary;
        
        view.style().background.color = Some(Theme::variant(variant.clone()));

        Self {
            id: Id::next().to_string(),
            label,
            variant,
            size: ButtonSize::Md,
            disabled: Signal::new(false),
            on_click: None,
            accessibility: Accessibility::default(),
            view,
        }
    }

    pub fn variant(self, v: Variant) -> Self { 
        self.view.style().background.color = Some(Theme::variant(v.clone()));
        self.view.mark_dirty();
        Self { variant: v, ..self }
    }

    pub fn on_click(mut self, f: impl Fn(rupa_vnode::UIEvent) + Send + Sync + 'static) -> Self { 
        self.on_click = Some(Arc::new(f)); 
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
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        let is_disabled = self.disabled.get();
        let mut node = VNode::element("button")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
            .with_attr("disabled", if is_disabled { "true" } else { "false" })
            .with_child(VNode::text(self.label.clone()));

        if !is_disabled {
            if let Some(ref handler) = self.on_click {
                let h = handler.clone();
                node = node.with_handler(move |e| h(e));
            }
        }

        node
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
        view.style().background.color = Some(rupa_vnode::Color::Rgba(1.0, 0.0, 0.0, 1.0));
        Self { id: Id::next().to_string(), view }
    }
}

impl Stylable for CloseButton { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() } }

impl Component for CloseButton {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    fn render(&self) -> VNode {
        VNode::element("button")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
            .with_child(VNode::text("×"))
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
        Self { id: Id::next().to_string(), children: Children::new(), view }
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
            handlers: Default::default(), 
            tag: "button-group".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: self.children.render_all(),
            key: Some(self.id.clone()),
        })
    }
}
