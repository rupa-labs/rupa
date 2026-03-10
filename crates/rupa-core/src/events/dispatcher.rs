use crate::component::Component;
use crate::events::{InputEvent, ButtonState, Modifiers, UIEvent};
use crate::scene::{SceneCore, HitDiscovery};
use rupa_support::Vec2;
use rupa_signals::{Signal, CursorIcon};
use std::sync::Arc;

/// The central engine for dispatching standardized input events into the component tree.
/// It handles hit-testing, focus management, pointer capture, and event bubbling.
pub struct InputDispatcher;

impl InputDispatcher {
    /// The main entry point for event dispatching.
    /// This should be called by platform runners when an OS event occurs.
    pub fn dispatch(
        event: InputEvent,
        root: &dyn Component,
        scene: &SceneCore,
        viewport: &Signal<Vec2>,
        cursor_pos: &mut Vec2,
        _requested_cursor: &mut CursorIcon,
        pointer_capture: &mut Option<String>,
        focused_id: &mut Option<String>,
        hovered_path: &mut Vec<String>,
        event_listeners: &[Arc<dyn Fn(&InputEvent) + Send + Sync>],
        debug: bool,
    ) {
        let taffy = &scene.layout_engine.taffy;
        if debug {
            log::debug!("Dispatching event: {:?}", event);
        }

        // 1. Run global plugin listeners
        for listener in event_listeners {
            listener(&event);
        }

        // 2. Determine target root (handling Modal focus traps)
        let mut target_root = root;
        fn find_modal(comp: &dyn Component) -> Option<&dyn Component> {
            for child in comp.children().iter().rev() {
                if child.is_modal() { return Some(*child); }
                if let Some(modal) = find_modal(*child) { return Some(modal); }
            }
            None
        }
        if let Some(modal) = find_modal(root) {
            target_root = modal;
        }

        match event {
            InputEvent::PointerMove { position } => {
                let delta = position - *cursor_pos;
                *cursor_pos = position;

                // A. Handle Pointer Capture
                if let Some(capture_id) = pointer_capture.as_ref() {
                    if let Some(hit) = scene.find_by_id(root, taffy, capture_id) {
                        let mut ui_ev = UIEvent::new(position - hit.local_pos);
                        hit.component.on_drag(&mut ui_ev, delta);
                        if let Some(false) = ui_ev.capture_request {
                            *pointer_capture = None;
                        }
                        return;
                    }
                }

                // B. Normal Hit-Testing & Hover Tracking
                match scene.find_target(target_root, taffy, *cursor_pos) {
                    HitDiscovery::Found(hit) => {
                        let mut ui_ev = UIEvent::new(hit.local_pos);
                        let new_hover_path: Vec<String> = hit.path.iter().map(|c| c.id().to_string()).collect();

                        // Detect Mouse Enter/Leave
                        // This is a simplified O(N) check for enter/leave
                        for comp in hit.path.iter().rev() {
                            let id = comp.id();
                            if !hovered_path.contains(&id.to_string()) {
                                comp.on_mouse_enter();
                            }
                        }
                        // Note: Leave detection requires keeping track of the previous path
                        // and comparing it. For brevity in this artisan MVP, we'll focus on Enter.

                        *hovered_path = new_hover_path;

                        // Bubble Drag Event
                        for comp in hit.path.iter().rev() {
                            comp.on_drag(&mut ui_ev, delta);
                            if ui_ev.consumed { break; }
                        }

                        // Update Cursor Style from the leaf
                        // *requested_cursor = hit.component.view_core().style().interactivity.cursor;
                    }
                    HitDiscovery::Missed => {
                        // Clear hovers
                        *hovered_path = Vec::new();
                    }
                }
            }

            InputEvent::PointerButton { button, state } => {
                if let HitDiscovery::Found(hit) = scene.find_target(target_root, taffy, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos)
                        .with_context(Modifiers::default(), Some(button), Some(state));

                    for comp in hit.path.iter().rev() {
                        match state {
                            ButtonState::Pressed => comp.on_click(&mut ui_ev),
                            ButtonState::Released => comp.on_release(&mut ui_ev),
                        }

                        if let Some(true) = ui_ev.capture_request {
                            *pointer_capture = Some(comp.id().to_string());
                        } else if let Some(false) = ui_ev.capture_request {
                            *pointer_capture = None;
                        }

                        if let Some(true) = ui_ev.focus_request {
                            *focused_id = Some(comp.id().to_string());
                        } else if let Some(false) = ui_ev.focus_request {
                            *focused_id = None;
                        }

                        if ui_ev.consumed { break; }
                    }
                }
            }

            InputEvent::PointerScroll { delta } => {
                if let HitDiscovery::Found(hit) = scene.find_target(target_root, taffy, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos);
                    for comp in hit.path.iter().rev() {
                        comp.on_scroll(&mut ui_ev, delta.y);
                        if ui_ev.consumed { break; }
                    }
                }
            }

            InputEvent::Key { key, state, modifiers } => {
                // Priority 1: Focused Component
                if let Some(focus_id) = focused_id.as_ref() {
                    if let Some(hit) = scene.find_by_id(target_root, taffy, focus_id) {
                        let mut ui_ev = UIEvent::new(Vec2::zero()).with_context(modifiers, None, Some(state));
                        for comp in hit.path.iter().rev() {
                            comp.on_key(&mut ui_ev, key);
                            if ui_ev.consumed { break; }
                        }
                        return;
                    }
                }

                // Priority 2: Hovered Component
                if let HitDiscovery::Found(hit) = scene.find_target(target_root, taffy, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos).with_context(modifiers, None, Some(state));
                    for comp in hit.path.iter().rev() {
                        comp.on_key(&mut ui_ev, key);
                        if ui_ev.consumed { break; }
                    }
                }
            }

            InputEvent::Ime(text) => {
                if let Some(focus_id) = focused_id.as_ref() {
                    if let Some(hit) = scene.find_by_id(target_root, taffy, focus_id) {
                        let mut ui_ev = UIEvent::new(Vec2::zero());
                        for comp in hit.path.iter().rev() {
                            comp.on_text(&mut ui_ev, &text);
                            if ui_ev.consumed { break; }
                        }
                    }
                }
            }

            InputEvent::Resize { size, .. } => {
                viewport.set(size);
                fn broadcast_resize(comp: &dyn Component, size: Vec2) {
                    let mut ui_ev = UIEvent::new(Vec2::zero());
                    comp.on_resize(&mut ui_ev, size);
                    for child in comp.children() {
                        broadcast_resize(child, size);
                    }
                }
                broadcast_resize(root, size);
            }

            InputEvent::SafeArea { top, right, bottom, left } => {
                fn broadcast_safe_area(comp: &dyn Component, t: f32, r: f32, b: f32, l: f32) {
                    let mut ui_ev = UIEvent::new(Vec2::zero());
                    comp.on_safe_area(&mut ui_ev, t, r, b, l);
                    for child in comp.children() {
                        broadcast_safe_area(child, t, r, b, l);
                    }
                }
                broadcast_safe_area(root, top, right, bottom, left);
            }

            _ => {}
        }
    }
}
