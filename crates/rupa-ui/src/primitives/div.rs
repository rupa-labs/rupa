use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_vnode::{Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

pub struct DivLogic<'a> {
    pub children: Children<'a>,
}

pub struct DivView {
    pub core: ViewCore,
}

pub struct Div<'a> {
    pub id: String,
    pub logic: DivLogic<'a>,
    pub view: DivView,
}

impl<'a> Div<'a> {
    pub fn new() -> Self {
        let view = DivView { core: ViewCore::new() };
        view.core.style().flex.flex_direction = rupa_vnode::FlexDirection::Col;
        Self {
            id: generate_id(),
            logic: DivLogic { children: Children::new() },
            view,
        }
    }

    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self {
        self.logic.children.push(child);
        self
    }
}

impl<'a> Component for Div<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "div".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            children: self.logic.children.render_all(),
            key: Some(self.id.clone()),
        })
    }


    fn get_node(&self) -> Option<rupa_core::scene::SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: rupa_core::scene::SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = taffy.new_with_children(style, &[]).unwrap();
        self.set_node(node.into());
        
        for child in self.logic.children.iter() {
            let child_node = child.layout(taffy, measurer, Some(node));
            taffy.add_child(node, child_node).unwrap();
        }

        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let style_ref = self.view.core.style.read().unwrap();
        
        if let Some(ref color) = style_ref.background.color {
            let layout = taffy.layout(node).unwrap();
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            );
        }

        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos, 0);
    }
}

impl<'a> Stylable for Div<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
