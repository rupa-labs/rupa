use rupa_core::{Component, VNode, Id};
use rupa_vnode::{Style, Theme, Color, Display};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

/// A visual separator between content.
pub struct Divider {
    pub id: String,
    pub orientation: Orientation,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Divider {
    pub fn new() -> Self {
        let mut style = Style::default();
        let theme = Theme::current();
        let palette = theme.active_palette();
        
        style.layout.display = Display::Block;
        style.sizing.height = Some(1.0);
        style.sizing.width = Some(-1.0); // 100%
        style.background.color = Some(palette.border.clone());
        
        Self {
            id: Id::next().to_string(),
            orientation: Orientation::Horizontal,
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }

    pub fn vertical(self) -> Self {
        {
            let mut style = self.get_style().write().unwrap();
            style.sizing.width = Some(1.0);
            style.sizing.height = Some(-1.0); // 100%
            style.layout.display = Display::InlineBlock;
        }
        Self { orientation: Orientation::Vertical, ..self }
    }

    pub fn color(self, color: Color) -> Self {
        self.get_style().write().unwrap().background.color = Some(color);
        self
    }
}

impl Component for Divider {
    fn id(&self) -> &str { &self.id }
    fn style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("divider")
            .with_style(self.get_style().read().unwrap().clone())
            .with_key(self.id.clone())
    }
}

impl Stylable for Divider {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
