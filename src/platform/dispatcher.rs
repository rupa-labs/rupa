use crate::core::component::Component;
use crate::support::vector::Vec2;
use crate::platform::events::{InputEvent, PointerButton, ButtonState, Modifiers};
use crate::scene::{SceneCore, HitDiscovery};
use std::sync::Arc;

/// The UIEvent passed to components during dispatch.
/// Contains rich context about the user interaction.
pub struct UIEvent {
    pub consumed: bool,
    pub local_pos: Vec2,
    pub modifiers: Modifiers,
    pub button: Option<PointerButton>,
    pub button_state: Option<ButtonState>,
    pub capture_request: Option<bool>,
    pub focus_request: Option<bool>, // Some(true) to request focus, Some(false) to blur
}

impl UIEvent {
    pub fn new(local_pos: Vec2) -> Self {
        Self { 
            consumed: false, 
            local_pos,
            modifiers: Modifiers::default(),
            button: None,
            button_state: None,
            capture_request: None,
            focus_request: None,
        }
    }

    pub fn with_context(mut self, modifiers: Modifiers, button: Option<PointerButton>, state: Option<ButtonState>) -> Self {
        self.modifiers = modifiers;
        self.button = button;
        self.button_state = state;
        self
    }

    pub fn consume(&mut self) { self.consumed = true; }
    pub fn capture_pointer(&mut self) { self.capture_request = Some(true); }
    pub fn release_pointer(&mut self) { self.capture_request = Some(false); }
    pub fn request_focus(&mut self) { self.focus_request = Some(true); }
    pub fn blur(&mut self) { self.focus_request = Some(false); }
}

pub struct InputDispatcher;

impl InputDispatcher {
    /// The central entry point for dispatching standardized input events into the component tree.
    pub fn dispatch(
        event: InputEvent,
        root: &dyn Component,
        scene: &SceneCore,
        cursor_pos: &mut Vec2,
        pointer_capture: &mut Option<String>,
        focused_id: &mut Option<String>,
        event_listeners: &[Arc<dyn Fn(&InputEvent) + Send + Sync>],
        debug: bool,
    ) {
        if debug {
            log::debug!("Dispatching event: {:?}", event);
        }
        // Trigger all registered plugin hooks first
    ...
            listener(&event);
        }

        match event {
            InputEvent::PointerMove { position } => {
                let delta = position - *cursor_pos;
                *cursor_pos = position;

                // Handle Pointer Capture
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

                if let HitDiscovery::Found(hit) = scene.find_target(root, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos);
                    for comp in hit.path.iter().rev() {
                        comp.on_drag(&mut ui_ev, delta); 
                        if ui_ev.consumed { break; }
                    }
                }
            }
            InputEvent::PointerButton { button, state } => {
                if let HitDiscovery::Found(hit) = scene.find_target(root, *cursor_pos) {
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

                        // Handle Focus Requests (Clicked element gets focus by default if it wants it)
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
                if let HitDiscovery::Found(hit) = scene.find_target(root, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos);
                    for comp in hit.path.iter().rev() {
                        comp.on_scroll(&mut ui_ev, delta.y);
                        if ui_ev.consumed { break; }
                    }
                }
            }
            InputEvent::Key { key, state, modifiers } => {
                // Priority 1: Component with Focus
                if let Some(focus_id) = focused_id.as_ref() {
                    if let Some(hit) = scene.find_by_id(root, focus_id) {
                        let mut ui_ev = UIEvent::new(Vec2::zero()) // Focus input doesn't necessarily have local_pos
                            .with_context(modifiers, None, Some(state));
                        
                        // Keyboard events bubble from focused child to parent
                        for comp in hit.path.iter().rev() {
                            comp.on_key(&mut ui_ev, key);
                            if ui_ev.consumed { break; }
                        }
                        return;
                    }
                }

                // Priority 2: Fallback to Hovered Component if no focus
                if let HitDiscovery::Found(hit) = scene.find_target(root, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos)
                        .with_context(modifiers, None, Some(state));
                    
                    for comp in hit.path.iter().rev() {
                        comp.on_key(&mut ui_ev, key); 
                        if ui_ev.consumed { break; }
                    }
                }
            }
            InputEvent::Ime(text) => {
                // IME (Text Input) goes to the focused component
                if let Some(focus_id) = focused_id.as_ref() {
                    if let Some(hit) = scene.find_by_id(root, focus_id) {
                        let mut ui_ev = UIEvent::new(Vec2::zero());
                        for comp in hit.path.iter().rev() {
                            comp.on_text(&mut ui_ev, &text);
                            if ui_ev.consumed { break; }
                        }
                    }
                }
            }
            InputEvent::Resize { size, .. } => {
                // Resize events are broadcasted to the whole tree
                // This is a simple recursive broadcast
                fn broadcast_resize(comp: &dyn Component, size: Vec2) {
                    let mut ui_ev = UIEvent::new(Vec2::zero());
                    comp.on_resize(&mut ui_ev, size);
                    for child in comp.children() {
                        broadcast_resize(child, size);
                    }
                }
                broadcast_resize(root, size);
            }
            InputEvent::Quit => {
                // Handle application teardown logic if needed.
            }
            _ => {}
        }
    }
}
