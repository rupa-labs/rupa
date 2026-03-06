use taffy::prelude::*;
use crate::core::component::Component;
use crate::utils::vector::Vec2;
use crate::platform::events::UIEvent;

pub struct HitTestResult<'a> {
    /// The path from the hit target up to the root.
    /// Index 0 is the target, last index is the root.
    pub path: Vec<&'a dyn Component>,
    pub local_pos: Vec2,
}

pub struct EventDispatcher;

impl EventDispatcher {
    /// Find the target component and its ancestors at the given position.
    pub fn hit_test<'a>(
        taffy: &TaffyTree<()>,
        root: &'a dyn Component,
        root_node: NodeId,
        cursor_pos: Vec2,
        parent_global_pos: Vec2,
    ) -> Option<HitTestResult<'a>> {
        let layout = taffy.layout(root_node).ok()?;
        let global_pos = parent_global_pos + Vec2::new(layout.location.x, layout.location.y);

        let is_inside = cursor_pos.x >= global_pos.x 
            && cursor_pos.x <= global_pos.x + layout.size.width
            && cursor_pos.y >= global_pos.y 
            && cursor_pos.y <= global_pos.y + layout.size.height;

        if !is_inside {
            return None;
        }

        // Try children first (they are on top)
        let children = root.children();
        let taffy_children = taffy.children(root_node).ok()?;

        for (i, child) in children.iter().enumerate().rev() {
            if let Some(child_node) = taffy_children.get(i) {
                if let Some(mut result) = Self::hit_test(taffy, *child, *child_node, cursor_pos, global_pos) {
                    result.path.push(root);
                    return Some(result);
                }
            }
        }

        // Target found
        Some(HitTestResult {
            path: vec![root],
            local_pos: cursor_pos - global_pos,
        })
    }

    pub fn dispatch_click(hit: HitTestResult) {
        let mut event = UIEvent::new();
        for component in hit.path {
            component.on_click(&mut event);
            if event.consumed { break; }
        }
    }

    pub fn dispatch_scroll(hit: HitTestResult, delta: f32) {
        let mut event = UIEvent::new();
        for component in hit.path {
            component.on_scroll(&mut event, delta);
            if event.consumed { break; }
        }
    }

    pub fn dispatch_drag(hit: HitTestResult, delta: Vec2) {
        let mut event = UIEvent::new();
        for component in hit.path {
            component.on_drag(&mut event, delta);
            if event.consumed { break; }
        }
    }
}
