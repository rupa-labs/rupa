use rupa_core::{Component, VNode, VElement, ViewCore, Id, Signal};
use rupa_vnode::{Style, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use std::sync::{RwLockWriteGuard, Arc};

pub struct BodyLogic<'a> {
    pub children: Children<'a>,
    pub overlays: Signal<Vec<&'a dyn Component>>,
}

pub struct BodyView {
    pub core: Arc<ViewCore>,
}

pub struct Body<'a> {
    pub id: String,
    pub logic: BodyLogic<'a>,
    pub view: BodyView,
}

impl<'a> Body<'a> {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            logic: BodyLogic {
                children: Children::new(),
                overlays: Signal::new(Vec::new()),
            },
            view: BodyView { core: Arc::new(ViewCore::new()) },
        }
    }

    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self {
        self.logic.children.push(child);
        self.view.core.mark_dirty();
        self
    }
}

impl<'a> Component for Body<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> {
        let mut all = self.logic.children.as_refs();
        let overlays = self.logic.overlays.get();
        for overlay in overlays {
            all.push(overlay);
        }
        all
    }

    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }

    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "body".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: {
                let mut children = self.logic.children.render_all();
                let overlays = self.logic.overlays.get();
                for o in overlays {
                    children.push(o.render());
                }
                children
            },
            key: Some(self.id.clone()),
        })
    }
}

impl<'a> Stylable for Body<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
