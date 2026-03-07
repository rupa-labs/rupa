use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_styling::{ColorMode, Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- LOGIC ---

pub struct ThemeSwitcherLogic {
    pub accessibility: Accessibility,
}

// --- VIEW ---

pub struct ThemeSwitcherView {
    pub core: ViewCore,
}

impl ThemeSwitcherView {
    pub fn new() -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self { core: view }
    }

    pub fn compute_layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let t_style = self.core.style().to_taffy();
        
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

    pub fn render(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        
        let mode = Theme::current().mode;
        let (label, bg_color) = match mode {
            ColorMode::Dark => ("🌙 DARK", Color::Semantic("surface".into(), None)),
            _ => ("☀️ LIGHT", Color::Semantic("surface".into(), None)),
        };

        // Draw background
        renderer.draw_rect(
            global_pos.x + layout.location.x, 
            global_pos.y + layout.location.y, 
            layout.size.width, 
            layout.size.height, 
            bg_color.to_rgba(), 
            8.0
        );

        // Draw text
        let text_color = Color::Semantic("text".into(), None).to_rgba();
        renderer.draw_text(
            label, 
            global_pos.x + layout.location.x + 12.0, 
            global_pos.y + layout.location.y + 8.0, 
            layout.size.width - 24.0,
            12.0, 
            text_color, 
            TextAlign::Left
        );
    }
}

// --- COMPONENT ---

pub struct ThemeSwitcher { 
    pub id: String, 
    pub logic: ThemeSwitcherLogic,
    pub view: ThemeSwitcherView,
}

impl ThemeSwitcher {
    pub fn new() -> Self {
        let view = ThemeSwitcherView::new();
        // Set default dimensions for the switcher
        {
            let mut style = view.core.style();
            style.sizing.width = Some(100.0);
            style.sizing.height = Some(32.0);
        }

        Self { 
            id: generate_id(), 
            logic: ThemeSwitcherLogic { 
                accessibility: Accessibility { role: SemanticRole::Button, ..Default::default() } 
            },
            view,
        }
    }
}

impl Stylable for ThemeSwitcher {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

impl Component for ThemeSwitcher {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "theme-switcher".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            children: vec![],
            key: Some(self.id.clone()),
        })
    }


    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        self.view.compute_layout(taffy, measurer, parent)
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        self.view.render(renderer, taffy, node, global_pos);
    }

    fn on_click(&self, _event: &mut UIEvent) {
        Theme::toggle_mode();
    }
}
