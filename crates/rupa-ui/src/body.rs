use rupa_core::{Style, generate_id, Vec2, state::Signal};
use rupa_core::component::Component;
use rupa_core::view::ViewCore;
use rupa_core::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::base::Stylable;
use rupa_core::scene::SceneNode;
use rupa_core::events::UIEvent;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

// --- LOGIC ---

/// The internal logic for the Body component.
/// Manages the primary content layer and the prioritized overlay stack.
pub struct BodyLogic<'a> {
    pub child: Option<Box<dyn Component + 'a>>,
    /// Prioritized overlays (Modals, Toasts, etc.)
    pub overlays: Arc<RwLock<Vec<Box<dyn Component + 'a>>>>,
    /// Reactive storage for safe area insets: (top, right, bottom, left)
    pub safe_area: Signal<(f32, f32, f32, f32)>,
    /// Reactive storage for viewport size
    pub viewport: Signal<Vec2>,
    /// Reactive storage for backdrop color (dimmer)
    pub backdrop_color: Signal<[f32; 4]>,
}

impl<'a> BodyLogic<'a> {
    pub fn new(child: Option<Box<dyn Component + 'a>>) -> Self {
        Self {
            child,
            overlays: Arc::new(RwLock::new(Vec::new())),
            safe_area: Signal::new((0.0, 0.0, 0.0, 0.0)),
            viewport: Signal::new(Vec2::zero()),
            backdrop_color: Signal::new([0.0, 0.0, 0.0, 0.5]), // Default semi-transparent black
        }
    }

    /// Add a new component to the global overlay stack.
    pub fn add_overlay(&self, component: Box<dyn Component + 'a>) {
        if let Ok(mut stack) = self.overlays.write() {
            stack.push(component);
        }
    }

    /// Clear all active overlays.
    pub fn clear_overlays(&self) {
        if let Ok(mut stack) = self.overlays.write() {
            stack.clear();
        }
    }
}

// --- VIEW ---

pub struct BodyView {
    pub core: ViewCore,
}

// --- BRIDGE (COMPONENT) ---

pub struct Body<'a> {
    pub id: String,
    pub logic: BodyLogic<'a>,
    pub view: BodyView,
}

impl<'a> Body<'a> {
    pub fn new(style: Style, child: Option<Box<dyn Component + 'a>>) -> Self {
        let mut final_style = style;
        if final_style.layout.display == crate::support::Display::None {
            final_style.layout.display = crate::support::Display::Flex;
        }
        final_style.sizing.width = Some(100.0);
        final_style.sizing.height = Some(100.0);

        Self {
            id: format!("root-body-{}", generate_id()),
            logic: BodyLogic::new(child),
            view: BodyView { core: ViewCore::new(final_style) },
        }
    }
}

impl<'a> Stylable for Body<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() }
}

impl<'a> Component for Body<'a> {
    fn id(&self) -> &str { &self.id }
    
    fn children(&self) -> Vec<&dyn Component> {
        let mut all = Vec::new();
        if let Some(ref c) = self.logic.child {
            all.push(c.as_ref());
        }
        if let Ok(stack) = self.logic.overlays.read() {
            for overlay in stack.iter() {
                all.push(overlay.as_ref());
            }
        }
        all
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { 
        let mut dirty = self.view.core.is_dirty();
        if !dirty {
            if let Ok(stack) = self.logic.overlays.read() {
                dirty = stack.iter().any(|o| o.is_dirty());
            }
        }
        dirty
    }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn on_safe_area(&self, _event: &mut UIEvent, top: f32, right: f32, bottom: f32, left: f32) {
        self.logic.safe_area.set((top, right, bottom, left));
        self.mark_dirty();
    }

    fn on_resize(&self, _event: &mut UIEvent, size: Vec2) {
        self.logic.viewport.set(size);
        self.mark_dirty();
    }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let mut style = self.view.core.get_style_mut().to_taffy();
        let (t, r, b, l) = self.logic.safe_area.get();
        style.padding = Rect { left: length(l), right: length(r), top: length(t), bottom: length(b) };

        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(style, &[]).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };

        let mut child_nodes = Vec::new();
        // 1. Primary Child Layout
        if let Some(ref child) = self.logic.child {
            child_nodes.push(child.layout(taffy, measurer, Some(node)));
        }
        
        // 2. Overlays Layout (Usually absolutely positioned or centered via Flex)
        if let Ok(stack) = self.logic.overlays.read() {
            for overlay in stack.iter() {
                child_nodes.push(overlay.layout(taffy, measurer, Some(node)));
            }
        }

        taffy.set_children(node, &child_nodes).unwrap();
        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.read().unwrap();
        
        // Background
        if let Some(color) = style_ref.background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style_ref.rounding.nw);
        }

        // 1. Paint Primary Content
        if let Some(ref child) = self.logic.child {
            child.paint(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos);
        }

        // 2. Backdrop & Overlays
        if let Ok(stack) = self.logic.overlays.read() {
            let has_modal = stack.iter().any(|o| o.is_modal());
            
            if has_modal {
                // Draw global backdrop
                let color = self.logic.backdrop_color.get();
                renderer.draw_rect(
                    global_pos.x, 
                    global_pos.y, 
                    layout.size.width, 
                    layout.size.height, 
                    color, 
                    0.0
                );
            }

            for overlay in stack.iter() {
                overlay.paint(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos);
            }
        }
    }
}
