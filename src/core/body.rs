use crate::support::{Style, generate_id, Vec2};
use crate::core::component::Component;
use crate::core::ViewCore;
use crate::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::base::Stylable;
use crate::scene::SceneNode;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

/// The internal root element of every Rupaui application.
/// Acts as the '<body>' equivalent, filling the entire window.
pub struct Body<'a> {
    pub id: String,
    pub view: BodyView,
    pub child: Option<Box<dyn Component + 'a>>,
}

pub struct BodyView {
    pub core: ViewCore,
}

impl<'a> Body<'a> {
    pub fn new(style: Style, child: Option<Box<dyn Component + 'a>>) -> Self {
        let mut final_style = style;
        // Ensure Body always takes full space
        final_style.sizing.width = Some(100.0);
        final_style.sizing.height = Some(100.0);
        final_style.layout.display = crate::support::Display::Flex;

        Self {
            id: format!("root-body-{}", generate_id()),
            view: BodyView { core: ViewCore::new(final_style) },
            child,
        }
    }
}

impl<'a> Stylable for Body<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() }
}

impl<'a> Component for Body<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> {
        self.child.as_ref().map(|c| c.as_ref() as &dyn Component).into_iter().collect()
    }
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

        let mut child_nodes = Vec::new();
        if let Some(ref child) = self.child {
            child_nodes.push(child.layout(taffy, measurer, Some(node)));
        }
        taffy.set_children(node, &child_nodes).unwrap();
        
        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.read().unwrap();
        
        if let Some(color) = style_ref.background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style_ref.rounding.nw);
        }

        if let Some(ref child) = self.child {
            child.paint(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos);
        }
    }
}
