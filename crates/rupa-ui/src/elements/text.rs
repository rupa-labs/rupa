use rupa_core::{Style, generate_id, Theme, Color, Attributes, Accessibility, Vec2};
use rupa_core::typography::TextAlign;
use rupa_core::component::Component;
use rupa_core::view::ViewCore;
use rupa_core::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::base::Stylable;
use rupa_core::scene::SceneNode;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- LOGIC ---

pub struct TextLogic {
    pub content: String,
    pub attributes: Attributes,
    pub accessibility: Accessibility,
}

// --- VIEW ---

pub struct TextView {
    pub core: ViewCore,
}

impl TextView {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self { core: ViewCore::new(style) }
    }

    pub fn compute_layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let t_style = self.core.get_style_mut().to_taffy();
        
        let node = if let Some(existing) = self.core.get_node() {
            if self.core.is_dirty() { taffy.set_style(existing.raw(), t_style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(t_style).unwrap();
            self.core.set_node(SceneNode::from(new_node));
            new_node
        };

        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }

        self.core.clear_dirty();
        node
    }

    pub fn render(&self, renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, logic: &TextLogic, global_pos: Vec2, width: f32) {
        let style = self.core.style.read().unwrap();
        let color = style.typography.color.clone().unwrap_or(Color::Semantic("text".into(), None)).to_rgba();
        renderer.draw_text(&logic.content, global_pos.x, global_pos.y, width, 16.0, color, TextAlign::Left);
    }
}

// --- COMPONENT ---

pub struct Text { 
    pub id: String, 
    pub logic: TextLogic,
    pub view: TextView,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self { 
            id: generate_id(), 
            logic: TextLogic { content: content.into(), attributes: Attributes::default(), accessibility: Accessibility::default() },
            view: TextView::new(),
        }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
}

impl Stylable for Text {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() }
}

impl Component for Text {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    
    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        self.view.compute_layout(taffy, measurer, parent)
    }
    
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        self.view.render(renderer, taffy, node, &self.logic, global_pos, layout.size.width);
    }
}
