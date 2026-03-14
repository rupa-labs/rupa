use rupa_core::{Component, VNode, Id};
use rupa_vnode::{Style, TextAlign, Spacing};
use crate::style::modifiers::Stylable;
use crate::elements::Children;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// A container that draws a border with a text label on top.
pub struct Fieldset<'a> {
    pub id: String,
    pub label: String,
    pub label_align: TextAlign,
    pub children: Children<'a>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl<'a> Fieldset<'a> {
    pub fn new(label: impl Into<String>) -> Self {
        let mut style = Style::default();
        style.border.width = 1.0;
        style.padding = Spacing::all(1.0);
        
        Self {
            id: Id::next().to_string(),
            label: label.into(),
            label_align: TextAlign::Left,
            children: Children::new(),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }

    pub fn child(mut self, child: impl Component + 'a) -> Self {
        self.children.push(std::boxed::Box::new(child));
        self
    }

    pub fn label_align(mut self, align: TextAlign) -> Self {
        self.label_align = align;
        self
    }
}

impl<'a> Component for Fieldset<'a> {
    fn id(&self) -> &str { &self.id }
    fn style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    
    fn render(&self) -> VNode {
        VNode::element("fieldset")
            .with_style(self.style().read().unwrap().clone())
            .with_children(self.children.render_all())
            .with_key(self.id.clone())
            .with_label(self.label.clone())
            .with_label_align(self.label_align)
    }
}

impl<'a> Stylable for Fieldset<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
