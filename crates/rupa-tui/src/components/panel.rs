use rupa_core::{Component, VNode, Id};
use rupa_vnode::Style;
use rupa_ui::style::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// A terminal-specific panel with customizable borders.
pub struct Panel<'a> {
    pub id: String,
    pub title: String,
    pub children: Vec<Box<dyn Component + 'a>>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl<'a> Panel<'a> {
    pub fn new(title: impl Into<String>) -> Self {
        let mut style = Style::default();
        // Default terminal border style
        style.border.width = rupa_vnode::types::Unit::Absolute(1.0);
        
        Self {
            id: Id::next().to_string(),
            title: title.into(),
            children: Vec::new(),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }

    pub fn child(mut self, child: impl Component + 'a) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl<'a> Component for Panel<'a> {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn children(&self) -> Vec<&dyn Component> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }

    fn render(&self) -> VNode {
        VNode::element("panel")
            .with_style(self.style.read().unwrap().clone())
            .with_attr("title", self.title.clone())
            .with_children(self.children.iter().map(|c| c.render()).collect())
            .with_key(self.id.clone())
            .with_label(self.title.clone()) // Automatically use title as border label
    }
}

impl<'a> Stylable for Panel<'a> {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
