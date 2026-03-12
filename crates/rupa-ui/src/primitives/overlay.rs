use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, Id, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Attributes};

use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

// --- LOGIC ---
pub struct OverlayLogic<'a> {
    pub children: Children<'a>,
}

// --- VIEW ---
pub struct OverlayView {
    pub core: Arc<ViewCore>,
}

impl OverlayView {
    pub fn new() -> Self {
        let core = Arc::new(ViewCore::new());
        core.style().layout.position = rupa_vnode::Position::Absolute;
        Self { core }
    }
}

// --- COMPONENT ---
pub struct Overlay<'a> {
    pub id: String,
    pub logic: OverlayLogic<'a>,
    pub view: OverlayView,
}

impl<'a> Overlay<'a> {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            logic: OverlayLogic { children: Children::new() },
            view: OverlayView::new(),
        }
    }
    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self { self.logic.children.push(c); self.view.core.mark_dirty(); self }
}

impl<'a> Stylable for Overlay<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

impl<'a> Component for Overlay<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { handlers: Default::default(), 
            tag: "overlay".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: self.logic.children.render_all(),
            key: Some(self.id.clone()),
        })
    }

    fn as_any(&self) -> Option<&dyn std::any::Any> { None }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { 
                taffy.set_style(existing.raw(), style).unwrap(); 
            }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(style, &[]).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };

        let child_nodes = self.logic.children.layout_all(taffy, measurer);
        taffy.set_children(node, &child_nodes).unwrap();
        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let style_ref = self.view.core.style.read().unwrap();
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos, 0);
    }
}
