use rupa_core::{Component, VNode, VElement, ViewCore, Id};
use rupa_vnode::{Style, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use std::sync::{RwLockWriteGuard, Arc};

/// A primitive for absolute-positioned elements.
pub struct Overlay<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub view: Arc<ViewCore>,
}

impl<'a> Overlay<'a> {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        view.style().layout.position = rupa_vnode::Position::Absolute;
        Self {
            id: Id::next().to_string(),
            children: Children::new(),
            view,
        }
    }

    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self {
        self.children.push(child);
        self.view.mark_dirty();
        self
    }
}

impl<'a> Component for Overlay<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "overlay".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: self.children.render_all(),
            key: Some(self.id.clone()),
        })
    }
}

impl<'a> Stylable for Overlay<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
