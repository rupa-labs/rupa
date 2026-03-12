use rupa_core::{Component, VNode, VElement, ViewCore, Id, Signal};
use rupa_vnode::{Style, Color, FontWeight};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextVariant {
    #[default]
    Plain,
    Info,
    Success,
    Warning,
    Error,
    Bold,
    Dim,
}

pub struct Text {
    pub id: String,
    pub content: Signal<String>,
    pub variant: Signal<TextVariant>,
    pub view: Arc<ViewCore>,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            id: Id::next().to_string(),
            content: Signal::new(content.into()),
            variant: Signal::new(TextVariant::Plain),
            view: Arc::new(ViewCore::new()),
        }
    }

    pub fn variant(self, variant: TextVariant) -> Self {
        self.variant.set(variant);
        self
    }

    // Helper builders
    pub fn plain(content: impl Into<String>) -> Self { Self::new(content) }
    pub fn info(content: impl Into<String>) -> Self { Self::new(content).variant(TextVariant::Info) }
    pub fn success(content: impl Into<String>) -> Self { Self::new(content).variant(TextVariant::Success) }
    pub fn warning(content: impl Into<String>) -> Self { Self::new(content).variant(TextVariant::Warning) }
    pub fn error(content: impl Into<String>) -> Self { Self::new(content).variant(TextVariant::Error) }
    pub fn bold(content: impl Into<String>) -> Self { Self::new(content).variant(TextVariant::Bold) }
    pub fn dim(content: impl Into<String>) -> Self { Self::new(content).variant(TextVariant::Dim) }
}

impl Component for Text {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        let variant = self.variant.get();
        let mut style = self.view.style().clone();

        // Map variants to standard artisan colors
        match variant {
            TextVariant::Info => style.typography.color = Some(Color::Rgba(0.2, 0.6, 1.0, 1.0)),
            TextVariant::Success => style.typography.color = Some(Color::Rgba(0.2, 0.8, 0.2, 1.0)),
            TextVariant::Warning => style.typography.color = Some(Color::Rgba(1.0, 0.8, 0.0, 1.0)),
            TextVariant::Error => style.typography.color = Some(Color::Rgba(1.0, 0.2, 0.2, 1.0)),
            TextVariant::Bold => style.typography.weight = FontWeight::Bold,
            TextVariant::Dim => style.typography.color = Some(Color::Rgba(0.5, 0.5, 0.5, 1.0)),
            _ => {}
        }

        VNode::Element(VElement {
            tag: "span".to_string(),
            style,
            attributes: Default::default(),
            handlers: Default::default(),
            motion: None,
            children: vec![VNode::text(self.content.get())],
            key: Some(self.id.clone()),
        })
    }
}

impl Stylable for Text {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
