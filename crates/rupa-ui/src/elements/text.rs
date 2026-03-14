use rupa_core::{Component, VNode, Id, Signal};
use rupa_vnode::{Style, Color, FontWeight, TextAlign};
use rupa_vnode::style::typography::{TextDecoration, TextTransform, FontFamily, TextWrap, TextOverflow};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

pub struct Text {
    pub id: String,
    pub content: Signal<String>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        let mut style = Style::default();
        rupa_vnode::style::theme::Theme::current().apply_defaults(&mut style);
        
        Self {
            id: Id::next().to_string(),
            content: Signal::new(content.into()),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }

    // --- Heading Presets ---
    pub fn h1(self) -> Self { self.size(48.0).black() }
    pub fn h2(self) -> Self { self.size(36.0).bold() }
    pub fn h3(self) -> Self { self.size(30.0).bold() }
    pub fn h4(self) -> Self { self.size(24.0).semibold() }
    pub fn h5(self) -> Self { self.size(20.0).semibold() }
    pub fn h6(self) -> Self { self.size(16.0).semibold() }

    // --- Semantic Presets ---
    pub fn lead(self) -> Self { self.size(20.0).light() }
    pub fn small(self) -> Self { self.size(12.0).muted() }
    pub fn mono(self) -> Self {
        self.style.write().unwrap().typography.family = FontFamily::Mono;
        self
    }

    // --- Weight Variants ---
    pub fn thin(self) -> Self { self.weight(FontWeight::Thin) }
    pub fn light(self) -> Self { self.weight(FontWeight::Light) }
    pub fn normal(self) -> Self { self.weight(FontWeight::Normal) }
    pub fn medium(self) -> Self { self.weight(FontWeight::Medium) }
    pub fn semibold(self) -> Self { self.weight(FontWeight::SemiBold) }
    pub fn bold(self) -> Self { self.weight(FontWeight::Bold) }
    pub fn black(self) -> Self { self.weight(FontWeight::Black) }

    fn weight(self, weight: FontWeight) -> Self {
        self.style.write().unwrap().typography.weight = weight;
        self
    }

    // --- Style & Decoration ---
    pub fn italic(self) -> Self {
        self.style.write().unwrap().typography.italic = true;
        self
    }

    pub fn underline(self) -> Self {
        self.style.write().unwrap().typography.decoration = TextDecoration::Underline;
        self
    }

    pub fn strikethrough(self) -> Self {
        self.style.write().unwrap().typography.decoration = TextDecoration::LineThrough;
        self
    }

    // --- Transform ---
    pub fn uppercase(self) -> Self {
        self.style.write().unwrap().typography.transform = TextTransform::Uppercase;
        self
    }

    pub fn lowercase(self) -> Self {
        self.style.write().unwrap().typography.transform = TextTransform::Lowercase;
        self
    }

    pub fn capitalize(self) -> Self {
        self.style.write().unwrap().typography.transform = TextTransform::Capitalize;
        self
    }

    // --- Alignment ---
    pub fn left(self) -> Self { self.align(TextAlign::Left) }
    pub fn center(self) -> Self { self.align(TextAlign::Center) }
    pub fn right(self) -> Self { self.align(TextAlign::Right) }
    pub fn justify(self) -> Self { self.align(TextAlign::Justify) }

    fn align(self, align: TextAlign) -> Self {
        self.style.write().unwrap().typography.align = align;
        self
    }

    // --- Layout Behavior ---
    pub fn wrap(self) -> Self {
        self.style.write().unwrap().typography.wrap = TextWrap::Wrap;
        self
    }

    pub fn no_wrap(self) -> Self {
        self.style.write().unwrap().typography.wrap = TextWrap::NoWrap;
        self
    }

    pub fn truncate(self) -> Self {
        let mut style = self.style.write().unwrap();
        style.typography.overflow = TextOverflow::Ellipsis;
        style.typography.wrap = TextWrap::NoWrap;
        drop(style);
        self
    }

    // --- Color Utilities ---
    pub fn primary(self) -> Self { self.color(Color::Semantic("primary".into(), None)) }
    pub fn success(self) -> Self { self.color(Color::Semantic("success".into(), None)) }
    pub fn warning(self) -> Self { self.color(Color::Semantic("warning".into(), None)) }
    pub fn error(self) -> Self { self.color(Color::Semantic("danger".into(), None)) }
    pub fn muted(self) -> Self { self.color(Color::Semantic("text-muted".into(), None)) }
    pub fn dim(self) -> Self { self.color(Color::Semantic("text-dim".into(), None)) }

    pub fn color(self, color: Color) -> Self {
        self.style.write().unwrap().typography.color = Some(color);
        self
    }

    pub fn size(self, size: f32) -> Self {
        self.style.write().unwrap().typography.size = rupa_vnode::types::Unit::Absolute(size);
        self
    }
}

impl Component for Text {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("span")
            .with_style(self.style.read().unwrap().clone())
            .with_child(VNode::text(self.content.get()))
            .with_key(self.id.clone())
    }
}

impl Stylable for Text {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
