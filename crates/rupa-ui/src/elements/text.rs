use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_vnode::{Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

pub struct TextLogic {
    pub content: Signal<String>,
}

pub struct TextView {
    pub core: ViewCore,
}

impl TextView {
    pub fn new() -> Self {
        Self { core: ViewCore::new() }
    }

    pub fn render(&self, renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, node: NodeId, logic: &TextLogic, global_pos: Vec2, width: f32) {
        let style = self.core.style.read().unwrap();
        let color = style.typography.color.clone().unwrap_or(Color::Rgba(1.0, 1.0, 1.0, 1.0));
        let layout = _taffy.layout(node).unwrap();
        
        renderer.draw_text(
            &logic.content.get(),
            global_pos.x + layout.location.x,
            global_pos.y + layout.location.y,
            width,
            style.typography.size,
            color.to_rgba(),
            style.typography.align,
        );
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
            id: generate_id(),
            logic: TextLogic { content: Signal::new(content.into()) },
            view: TextView::new(),
        }
    }
}

impl Component for Text {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    fn render(&self) -> VNode { VNode::text(self.logic.content.get()) }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap();
        let t_style = style.to_taffy();
        
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), t_style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(t_style).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };

        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }

        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        self.view.render(renderer, taffy, node, &self.logic, global_pos, layout.size.width);
    }
}

impl Stylable for Text {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
