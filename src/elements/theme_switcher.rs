use crate::support::{Style, generate_id, Theme, ColorMode, Accessibility, Role, Attributes, Vec2};
use crate::core::component::Component;
use crate::renderer::renderer::Renderer;
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use taffy::prelude::*;
use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ThemeSwitcher { 
    pub id: String, 
    pub style: RefCell<Style>, 
    pub accessibility: Accessibility, 
    pub attributes: Attributes,
    node: Cell<Option<NodeId>>,
    dirty: AtomicBool,
}

impl ThemeSwitcher {
    pub fn new() -> Self {
        Self { 
            id: generate_id(), 
            style: RefCell::new(Style::default()), 
            accessibility: Accessibility { role: Role::Button, ..Default::default() }, 
            attributes: Attributes::default(),
            node: Cell::new(None),
            dirty: AtomicBool::new(true),
        }
    }
    pub fn cycle_mode() {
        Theme::update(|t| {
            t.mode = match t.mode {
                ColorMode::Dark => ColorMode::Light,
                ColorMode::Light => ColorMode::Dark,
                ColorMode::System => ColorMode::Dark,
            };
        });
    }
}

impl Stylable for ThemeSwitcher {
    fn get_style_mut(&self) -> &mut Style {
        unsafe { &mut *self.style.as_ptr() }
    }
}

impl Component for ThemeSwitcher {
    fn id(&self) -> &str { &self.id }
    
    fn children(&self) -> Vec<&dyn Component> { vec![] }

    fn get_node(&self) -> Option<NodeId> { self.node.get() }
    fn set_node(&self, node: NodeId) { self.node.set(Some(node)); }
    fn is_dirty(&self) -> bool { self.dirty.load(Ordering::Relaxed) }
    fn mark_dirty(&self) { self.dirty.store(true, Ordering::Relaxed); }
    fn clear_dirty(&self) { self.dirty.store(false, Ordering::Relaxed); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.get_node() {
            if self.is_dirty() {
                taffy.set_style(existing, self.style.borrow().to_taffy()).unwrap();
            }
            existing
        } else {
            let new_node = taffy.new_leaf(self.style.borrow().to_taffy()).unwrap();
            self.set_node(new_node);
            new_node
        };

        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) {
                taffy.add_child(p, node).unwrap();
            }
        }

        self.clear_dirty();
        node
    }
    
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, _render_pass: &mut wgpu::RenderPass<'_>, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let color = if Theme::current().mode == ColorMode::Dark { [0.2, 0.2, 0.2, 1.0] } else { [0.9, 0.9, 0.9, 1.0] };
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color, 4.0);
    }
    
    fn on_click(&self, _event: &mut UIEvent) { Self::cycle_mode(); }
    fn on_scroll(&self, _event: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _event: &mut UIEvent, _: Vec2) {}
}
