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

            _ => {}
        }
    }
}
