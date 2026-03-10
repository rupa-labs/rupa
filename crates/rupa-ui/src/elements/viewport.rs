use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_vnode::{Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- LOGIC ---
pub struct ViewportLogic<'a> { 
    pub child: Option<Box<dyn Component + 'a>> 
}

// --- VIEW ---
pub struct ViewportView { 
    pub core: ViewCore 
}

impl ViewportView {
    pub fn new() -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self { core: view }
    }
}

// --- COMPONENT ---
/// A semantic component representing a rectangular clipping area or a nested viewport.
pub struct Viewport<'a> { 
    pub id: String, 
    pub logic: ViewportLogic<'a>, 
    pub view: ViewportView 
}

impl<'a> Viewport<'a> {
    pub fn new() -> Self {
        Self { 
            id: generate_id(), 
            logic: ViewportLogic { child: None }, 
            view: ViewportView::new() 
        }
    }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { 
        self.logic.child = Some(child); 
        self.view.core.mark_dirty(); 
        self 
    }
}

impl<'a> Stylable for Viewport<'a> { 
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() } 
}

impl<'a> Component for Viewport<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { 
        self.logic.child.as_ref().map(|c| c.as_ref()).into_iter().collect() 
    }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "viewport".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            children: self.logic.child.as_ref().map(|c| vec![c.render()]).unwrap_or_default(),
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

        let mut child_nodes = Vec::new();
        if let Some(ref child) = self.logic.child {
            child_nodes.push(child.layout(taffy, measurer, Some(node)));
        }
        taffy.set_children(node, &child_nodes).unwrap();
        
        self.view.core.clear_dirty(); 
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap(); 
        let style_ref = self.view.core.style.read().unwrap();
        
        if let Some(ref color) = style_ref.background.color { 
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            ); 
        }

        if let Some(ref child) = self.logic.child {
            child.paint(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos);
        }
    }
}
