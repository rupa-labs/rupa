use rupa_core::{Component, VNode, ViewCore, Id, Signal};
use rupa_vnode::Style;
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc};

pub struct TextLogic {
    pub content: Signal<String>,
}

pub struct TextView {
    pub core: Arc<ViewCore>,
}

impl TextView {
    pub fn new() -> Self {
        Self { core: Arc::new(ViewCore::new()) }
    }
}

pub struct Text {
    pub id: String,
    pub logic: TextLogic,
    pub view: TextView,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            id: Id::next().to_string(),
            logic: TextLogic { content: Signal::new(content.into()) },
            view: TextView::new(),
        }
    }
}

impl Component for Text {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    fn render(&self) -> VNode { 
        VNode::text(self.logic.content.get())
            // Future: Support wrapping text in span/p with style
    }
}

impl Stylable for Text {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
