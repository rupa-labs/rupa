use crate::support::{Style, generate_id, Vec2, Position};
use crate::core::component::Component;
use crate::core::ViewCore;
use crate::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::base::Stylable;
use crate::scene::SceneNode;
use crate::elements::layout::container::Children;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- LOGIC ---
pub struct OverlayLogic<'a> {
    pub children: Children<'a>,
}

// --- VIEW ---
pub struct OverlayView {
    pub core: ViewCore,
}

impl OverlayView {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.layout.position = Position::Absolute;
        Self { core: ViewCore::new(style) }
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
            id: generate_id(),
            logic: OverlayLogic { children: Children::new() },
            view: OverlayView::new(),
        }
    }
    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self { self.logic.children.add(c); self.view.core.mark_dirty(); self }
}

impl<'a> Stylable for Overlay<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() }
}

impl<'a> Component for Overlay<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.get_all() }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { 
                taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); 
            }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(self.view.core.get_style_mut().to_taffy(), &[]).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };

        let child_nodes = self.logic.children.layout_all(taffy, measurer);
        taffy.set_children(node, &child_nodes).unwrap();
        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, _node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let style_ref = self.view.core.style.read().unwrap();
        self.logic.children.paint_all(renderer, taffy, _node, is_group_hovered || style_ref.is_group, global_pos, 0);
    }
}
