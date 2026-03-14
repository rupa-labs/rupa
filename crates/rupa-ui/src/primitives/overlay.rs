use rupa_core::{Component, VNode, Id, Signal, Vec2};
use rupa_vnode::{Style, Position};
use crate::style::modifiers::Stylable;
use crate::elements::Children;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// # Primitive: Overlay Foundation
/// 
/// The low-level engine for modals, tooltips, and popovers.
/// It provides the base logic for visibility and absolute positioning.
pub struct Overlay<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
    pub is_visible: Signal<bool>,
    pub position: Signal<Vec2>,
}

impl<'a> Overlay<'a> {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.layout.position = Position::Absolute;
        
        Self {
            id: Id::next().to_string(),
            children: Children::new(),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
            is_visible: Signal::new(false),
            position: Signal::new(Vec2::zero()),
        }
    }

    pub fn child(mut self, child: impl Component + 'a) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl<'a> Component for Overlay<'a> {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    
    fn render(&self) -> VNode {
        if !self.is_visible.get() {
            return VNode::Empty;
        }

        VNode::element("overlay")
            .with_style(self.style.read().unwrap().clone())
            .with_children(self.children.render_all())
            .with_key(self.id.clone())
    }
}

impl<'a> Stylable for Overlay<'a> {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
