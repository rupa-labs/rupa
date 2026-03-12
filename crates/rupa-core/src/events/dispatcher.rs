use rupa_vnode::VNode;
use crate::events::{InputEvent, ButtonState};
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
        let taffy = &scene.layout_engine.taffy;
        if debug {
            log::debug!("Dispatching event: {:?}", event);
        }

        for listener in event_listeners {
            listener(&event);
        }

        match event {
            InputEvent::PointerMove { position } => {
                let _delta = position - *cursor_pos;
                *cursor_pos = position;

                match scene.find_target(root_vnode, taffy, *cursor_pos) {
                    HitDiscovery::Found(hit) => {
                        let ui_ev = rupa_vnode::UIEvent::Pointer(rupa_vnode::PointerEvent {
                            pos: hit.local_pos,
                            delta: _delta,
                            button: None,
                            modifiers: rupa_vnode::Modifiers::default(),
                        });

                        // Trigger hover handlers
                        for node in hit.path.iter().rev() {
                            if let VNode::Element(el) = node {
                                if let Some(handler) = &el.handlers.on_hover {
                                    handler(ui_ev.clone());
                                }
                            }
                        }
                    }
                    HitDiscovery::Missed => {}
                }
            }

            InputEvent::PointerButton { button, state } => {
                if let HitDiscovery::Found(hit) = scene.find_target(root_vnode, taffy, *cursor_pos) {
                    let btn = match button {
                        crate::events::PointerButton::Primary => rupa_vnode::PointerButton::Primary,
                        crate::events::PointerButton::Secondary => rupa_vnode::PointerButton::Secondary,
                        crate::events::PointerButton::Auxiliary => rupa_vnode::PointerButton::Middle,
                        _ => rupa_vnode::PointerButton::Other(0),
                    };

                    let ui_ev = rupa_vnode::UIEvent::Pointer(rupa_vnode::PointerEvent {
                        pos: hit.local_pos,
                        delta: Vec2::zero(),
                        button: Some(btn),
                        modifiers: rupa_vnode::Modifiers::default(),
                    });

                    if state == ButtonState::Pressed {
                        for node in hit.path.iter().rev() {
                            if let VNode::Element(el) = node {
                                if let Some(handler) = &el.handlers.on_click {
                                    handler(ui_ev.clone());
                                }
                            }
                        }
                    }
                }
            }

            InputEvent::Resize { size, .. } => {
                viewport.set(size);
            }

            InputEvent::Key { key, state, .. } => {
                if key == crate::events::KeyCode::Enter && state == ButtonState::Pressed {
                    // If something is focused, trigger its click handler
                    if let Some(id) = _focused_id {
                        // In a real implementation, we would look up the node by ID in the Scene graph.
                        // For this artisan maturation, we search the VNode tree for the element with the matching key.
                        Self::trigger_click_by_id(root_vnode, id);
                    } else {
                        // Fallback: trigger the first clickable element (common in simple CLI wizards)
                        Self::trigger_first_click(root_vnode);
                    }
                }
            }

            _ => {}
        }
    }

    fn trigger_click_by_id(node: &VNode, id: &str) -> bool {
        match node {
            VNode::Element(el) => {
                if el.key.as_deref() == Some(id) {
                    if let Some(handler) = &el.handlers.on_click {
                        handler(rupa_vnode::UIEvent::Keyboard);
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
                    handler(rupa_vnode::UIEvent::Keyboard);
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
