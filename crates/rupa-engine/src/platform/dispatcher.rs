use rupa_core::component::Component;
use rupa_core::{Vec2, Signal, CursorIcon};
use rupa_core::events::{InputEvent, PointerButton, ButtonState, Modifiers, UIEvent}; 
use crate::scene::{SceneCore, HitDiscovery};
use std::sync::Arc;

pub struct InputDispatcher;

impl InputDispatcher {
    /// The central entry point for dispatching standardized input events into the component tree.
    pub fn dispatch(
        event: InputEvent,
        root: &dyn Component,
        scene: &SceneCore,
        viewport: &Signal<Vec2>,
        cursor_pos: &mut Vec2,
        requested_cursor: &mut CursorIcon,
        pointer_capture: &mut Option<String>,
        focused_id: &mut Option<String>,
        event_listeners: &[Arc<dyn Fn(&InputEvent) + Send + Sync>],
        debug: bool,
    ) {
        if debug {
            log::debug!("Dispatching event: {:?}", event);
        }
        
        // Trigger all registered plugin hooks first
        for listener in event_listeners {
            listener(&event);
        }

        // --- Focus Trap Detection (Agnostic version) ---
        let mut target_root = root;
        
        // Search for the topmost modal child recursively
        fn find_modal(comp: &dyn Component) -> Option<&dyn Component> {
            for child in comp.children().iter().rev() {
                if child.is_modal() {
                    return Some(*child);
                }
                if let Some(modal) = find_modal(*child) {
                    return Some(modal);
                }
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

                // Handle Pointer Capture (Capture overrides Focus Trap)
                if let Some(capture_id) = pointer_capture.as_ref() {
                    if let Some(hit) = scene.find_by_id(root, capture_id) {
                        let local_pos = position - hit.local_pos;
                        let mut ui_ev = UIEvent::new(local_pos);
                        hit.component.on_drag(&mut ui_ev, delta);
                        if let Some(false) = ui_ev.capture_request {
                             *pointer_capture = None;
                        }
                        return;
                    }
                }

                // Normal hit-testing
                if let HitDiscovery::Found(hit) = scene.find_target(target_root, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos);
                    for comp in hit.path.iter().rev() {
                        comp.on_drag(&mut ui_ev, delta); 
                        if ui_ev.consumed { break; }
                    }
                }
            }
            InputEvent::PointerButton { button, state } => {
                if let HitDiscovery::Found(hit) = scene.find_target(target_root, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos)
                        .with_context(Modifiers::default(), Some(button), Some(state));
                    
                    for comp in hit.path.iter().rev() {
                        match state {
                            ButtonState::Pressed => comp.on_click(&mut ui_ev),
                            ButtonState::Released => comp.on_release(&mut ui_ev),
                        }
                        
                        // Handle Capture Requests
                        if let Some(true) = ui_ev.capture_request {
                            *pointer_capture = Some(comp.id().to_string());
                        } else if let Some(false) = ui_ev.capture_request {
                            *pointer_capture = None;
                        }

                        // Handle Focus Requests
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
                if let HitDiscovery::Found(hit) = scene.find_target(target_root, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos);
                    for comp in hit.path.iter().rev() {
                        comp.on_scroll(&mut ui_ev, delta.y);
                        if ui_ev.consumed { break; }
                    }
                }
            }
            InputEvent::Key { key, state, modifiers } => {
                // Priority 1: Component with Focus (must be within target_root if trap is active)
                if let Some(focus_id) = focused_id.as_ref() {
                    if let Some(hit) = scene.find_by_id(target_root, focus_id) {
                        let mut ui_ev = UIEvent::new(Vec2::zero())
                            .with_context(modifiers, None, Some(state));
                        
                        for comp in hit.path.iter().rev() {
                            comp.on_key(&mut ui_ev, key);
                            if ui_ev.consumed { break; }
                        }
                        return;
                    }
                }

                // Priority 2: Hovered Component within target_root
                if let HitDiscovery::Found(hit) = scene.find_target(target_root, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos)
                        .with_context(modifiers, None, Some(state));
                    
                    for comp in hit.path.iter().rev() {
                        comp.on_key(&mut ui_ev, key); 
                        if ui_ev.consumed { break; }
                    }
                }
            }
            InputEvent::Ime(text) => {
                if let Some(focus_id) = focused_id.as_ref() {
                    if let Some(hit) = scene.find_by_id(target_root, focus_id) {
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
            InputEvent::Quit => {}
            _ => {}
        }
    }
}
