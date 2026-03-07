use crate::support::{Style, generate_id, Theme, Vec2};
use crate::core::component::Component;
use crate::core::ViewCore;
use crate::elements::layout::container::Children;
use crate::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::base::Stylable;
use crate::scene::SceneNode;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- LOGIC ---
pub struct SectionLogic<'a> { pub name: String, pub children: Children<'a> }
// --- VIEW ---
pub struct SectionView { pub core: ViewCore }
// --- COMPONENT ---
pub struct Section<'a> { pub id: String, pub logic: SectionLogic<'a>, pub view: SectionView }

impl<'a> Section<'a> {
    pub fn new(name: impl Into<String>) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), logic: SectionLogic { name: name.into(), children: Children::new() }, view: SectionView { core: ViewCore::new(style) } }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.logic.children.add(child); self.view.core.mark_dirty(); self }
}

impl<'a> Stylable for Section<'a> { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() } }

impl<'a> Component for Section<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.get_all() }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(self.view.core.get_style_mut().to_taffy(), &[]).unwrap(); self.view.core.set_node(SceneNode::from(new_node)); new_node
        };
        let child_nodes = self.logic.children.layout_all(taffy, measurer);
        taffy.set_children(node, &child_nodes).unwrap();
        self.view.core.clear_dirty(); node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap(); 
        let style_ref = self.view.core.style.read().unwrap();
        let style: &Style = if is_group_hovered && style_ref.group_hover.is_some() { 
            style_ref.group_hover.as_ref().unwrap() 
        } else { 
            &style_ref 
        };
        
        if let Some(color) = style.background.color.as_ref() { 
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw); 
        }
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered || style.is_group, global_pos, 0);
    }
}
