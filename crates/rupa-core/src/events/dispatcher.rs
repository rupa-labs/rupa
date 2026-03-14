use rupa_vnode::VNode;
use crate::events::{InputEvent, PointerAction, FocusAction, SystemEvent, ButtonState, UIEvent, InteractionState};
use crate::scene::{SceneCore, HitDiscovery};
use rupa_base::Vec2;
use rupa_signals::{Signal, CursorIcon};
use std::sync::Arc;

/// The central engine for dispatching standardized input events into the VNode tree.
pub struct InputDispatcher;

impl InputDispatcher {
    pub fn dispatch(
        event: InputEvent,
        root_vnode: &VNode,
        scene: &SceneCore,
        viewport: &Signal<Vec2>,
        cursor_pos: &mut Vec2,
        _requested_cursor: &mut CursorIcon,
        _pointer_capture: &mut Option<String>,
        _focused_id: &mut Option<String>,
        _hovered_path: &mut Vec<String>,
        event_listeners: &[Arc<dyn Fn(&InputEvent) + Send + Sync>],
        debug: bool,
    ) {
        if debug {
            log::debug!("Dispatching event: {:?}", event);
        }

        for listener in event_listeners {
            listener(&event);
        }

        match event {
            InputEvent::Pointer { position, action, button, modifiers } => {
                *cursor_pos = position;

                if let HitDiscovery::Found(hit) = scene.find_target(root_vnode, *cursor_pos) {
                    let mut ui_ev = UIEvent::new(hit.local_pos)
                        .with_context(modifiers, button, None);
                    
                    match action {
                        PointerAction::Move | PointerAction::Hover => {
                            // Sync Reactive State
                            if let Some(ref target_id) = hit.target_id {
                                InteractionState::global().hovered_id.set(Some(target_id.clone()));
                            } else {
                                InteractionState::global().hovered_id.set(None::<String>);
                            }

                            // Trigger hover handlers
                            for node in hit.path.iter().rev() {
                                if let VNode::Element(el) = node {
                                    if let Some(handler) = &el.handlers.on_hover {
                                        handler(ui_ev.clone());
                                        if ui_ev.consumed { break; }
                                    }
                                }
                            }
                        }
                        PointerAction::Down => {
                            let state = ButtonState::Pressed;
                            ui_ev = ui_ev.with_context(modifiers, button, Some(state));
                            
                            InteractionState::global().active_id.set(hit.target_id.clone());
                            if let Some(ref target_id) = hit.target_id {
                                InteractionState::global().focused_id.set(Some(target_id.clone()));
                                *_focused_id = Some(target_id.clone());
                            }

                            for node in hit.path.iter().rev() {
                                if let VNode::Element(el) = node {
                                    if let Some(handler) = &el.handlers.on_click {
                                        handler(ui_ev.clone());
                                        if ui_ev.consumed { break; }
                                    }
                                }
                            }
                        }
                        PointerAction::Up => {
                            InteractionState::global().active_id.set(None::<String>);
                        }
                        _ => {}
                    }
                } else {
                    if action == PointerAction::Down {
                        InteractionState::global().focused_id.set(None::<String>);
                        *_focused_id = None;
                    }
                    InteractionState::global().hovered_id.set(None::<String>);
                    InteractionState::global().active_id.set(None::<String>);
                }
            }

            InputEvent::System(sys_ev) => {
                match sys_ev {
                    SystemEvent::Resize { size, .. } => {
                        viewport.set(size);
                    }
                    _ => {}
                }
            }

            InputEvent::Key { key, state, modifiers: _ } => {
                if state == ButtonState::Pressed {
                    if let Some(id) = _focused_id {
                        match key {
                            crate::events::KeyCode::Enter => {
                                Self::trigger_submit_by_id(root_vnode, id);
                                Self::trigger_click_by_id(root_vnode, id);
                            }
                            crate::events::KeyCode::Char(c) => {
                                Self::trigger_input_by_id(root_vnode, id, Some(c), false);
                            }
                            crate::events::KeyCode::Backspace => {
                                Self::trigger_input_by_id(root_vnode, id, None, true);
                            }
                            _ => {}
                        }
                    } else if key == crate::events::KeyCode::Enter {
                        Self::trigger_first_click(root_vnode);
                    }
                }
            }

            InputEvent::Focus { action, .. } => {
                match action {
                    FocusAction::Next => {
                        // Logic to cycle focus to next element
                    }
                    FocusAction::Prev => {
                        // Logic to cycle focus to previous element
                    }
                    _ => {}
                }
            }
        }
    }

    fn trigger_submit_by_id(node: &VNode, id: &str) -> bool {
        match node {
            VNode::Element(el) => {
                if el.key.as_deref() == Some(id) {
                    if let Some(handler) = &el.handlers.on_submit {
                        let val = el.attributes.get("value").cloned().unwrap_or_default();
                        handler(val);
                        return true;
                    }
                }
                for child in &el.children {
                    if Self::trigger_submit_by_id(child, id) { return true; }
                }
            }
            VNode::Fragment(children) => {
                for child in children {
                    if Self::trigger_submit_by_id(child, id) { return true; }
                }
            }
            _ => {}
        }
        false
    }

    fn trigger_input_by_id(node: &VNode, id: &str, char: Option<char>, is_backspace: bool) -> bool {
        match node {
            VNode::Element(el) => {
                if el.key.as_deref() == Some(id) {
                    if let Some(handler) = &el.handlers.on_input {
                        let mut val = el.attributes.get("value").cloned().unwrap_or_default();
                        if is_backspace {
                            val.pop();
                        } else if let Some(c) = char {
                            val.push(c);
                        }
                        handler(val);
                        return true;
                    }
                }
                for child in &el.children {
                    if Self::trigger_input_by_id(child, id, char, is_backspace) { return true; }
                }
            }
            VNode::Fragment(children) => {
                for child in children {
                    if Self::trigger_input_by_id(child, id, char, is_backspace) { return true; }
                }
            }
            _ => {}
        }
        false
    }

    fn trigger_click_by_id(node: &VNode, id: &str) -> bool {
        match node {
            VNode::Element(el) => {
                if el.key.as_deref() == Some(id) {
                    if let Some(handler) = &el.handlers.on_click {
                        handler(UIEvent::new(Vec2::zero()));
                        return true;
                    }
                }
                for child in &el.children {
                    if Self::trigger_click_by_id(child, id) { return true; }
                }
            }
            VNode::Fragment(children) => {
                for child in children {
                    if Self::trigger_click_by_id(child, id) { return true; }
                }
            }
            _ => {}
        }
        false
    }

    fn trigger_first_click(node: &VNode) -> bool {
        match node {
            VNode::Element(el) => {
                if let Some(handler) = &el.handlers.on_click {
                    handler(UIEvent::new(Vec2::zero()));
                    return true;
                }
                for child in &el.children {
                    if Self::trigger_first_click(child) { return true; }
                }
            }
            VNode::Fragment(children) => {
                for child in children {
                    if Self::trigger_first_click(child) { return true; }
                }
            }
            _ => {}
        }
        false
    }
}
