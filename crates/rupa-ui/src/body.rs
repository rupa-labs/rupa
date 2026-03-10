use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Attributes};
use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use taffy::prelude::*;
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
            id: generate_id(),
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
            tag: "body".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: self.logic.children.render_all(),
            key: Some(self.id.clone()),
        })
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = taffy.new_with_children(style, &[]).unwrap();
        self.set_node(node.into());
        
        let mut child_nodes = self.logic.children.layout_all(taffy, measurer);

        let overlays = self.logic.overlays.get();
        for overlay in overlays {
            let overlay_node = overlay.layout(taffy, measurer, Some(node));
            child_nodes.push(overlay_node);
        }

        taffy.set_children(node, &child_nodes).unwrap();
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
                0.0
            );
        }

        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos, 0);
        
        let overlays = self.logic.overlays.get();
        for overlay in overlays {
            overlay.paint(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos);
        }
    }
}

impl<'a> Stylable for Body<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
