use rupa_core::{Component, VNode, VElement, ViewCore, Id};
use rupa_vnode::{Style, Attributes};
use crate::style::modifiers::base::Stylable;
use std::sync::{RwLockWriteGuard, Arc};

pub struct ThemeSwitcher {
    pub id: String,
    pub view: Arc<ViewCore>,
}

impl ThemeSwitcher {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl Component for ThemeSwitcher {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "theme-switcher".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: vec![],
            key: Some(self.id.clone()),
        })
    }
}

impl Stylable for ThemeSwitcher {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
