use rupa_core::VNode;
use rupa_core::events::InputEvent;
use rupa_core::events::dispatcher::InputDispatcher;
use rupa_engine::platform::SharedPlatformCore;

pub struct EventHandler;

impl EventHandler {
    pub fn dispatch(core_handle: &SharedPlatformCore, event: InputEvent, last_vnode: &VNode) {
        let mut core = match core_handle.write() {
            Ok(c) => c,
            Err(_) => return,
        };
        
        let mut cursor_pos = core.cursor_pos;
        let mut requested_cursor = core.requested_cursor;
        let mut pointer_capture = core.pointer_capture.take();
        let mut focused_id = core.focused_id.take();
        let mut hovered_path = std::mem::take(&mut core.hovered_path);
        let event_listeners = core.event_listeners.clone();

        // TUI Focus Management
        if let InputEvent::Key { key, state, .. } = &event {
            if *state == rupa_core::events::ButtonState::Pressed {
                let clickable_ids = Self::find_clickable_ids(last_vnode);
                if !clickable_ids.is_empty() {
                    let mut current_idx = focused_id.as_ref()
                        .and_then(|id| clickable_ids.iter().position(|c| c == id))
                        .unwrap_or(0);

                    match key {
                        rupa_core::events::KeyCode::ArrowDown | rupa_core::events::KeyCode::Tab => {
                            current_idx = (current_idx + 1) % clickable_ids.len();
                            focused_id = Some(clickable_ids[current_idx].clone());
                            core.focused_id = focused_id.clone();
                            return;
                        }
                        rupa_core::events::KeyCode::ArrowUp => {
                            current_idx = (current_idx + clickable_ids.len() - 1) % clickable_ids.len();
                            focused_id = Some(clickable_ids[current_idx].clone());
                            core.focused_id = focused_id.clone();
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }
        
        InputDispatcher::dispatch(
            event,
            last_vnode,
            &core.scene,
            &core.viewport,
            &mut cursor_pos,
            &mut requested_cursor,
            &mut pointer_capture,
            &mut focused_id,
            &mut hovered_path,
            &event_listeners,
            core.debug,
        );

        core.cursor_pos = cursor_pos;
        core.requested_cursor = requested_cursor;
        core.pointer_capture = pointer_capture;
        core.focused_id = focused_id;
        core.hovered_path = hovered_path;
    }

    fn find_clickable_ids(node: &VNode) -> Vec<String> {
        let mut ids = Vec::new();
        match node {
            VNode::Element(el) => {
                // Focusable if it has handlers or is an input
                if el.handlers.on_click.is_some() || el.handlers.on_input.is_some() || el.tag == "input" {
                    if let Some(ref key) = el.key {
                        ids.push(key.clone());
                    }
                }
                for child in &el.children {
                    ids.extend(Self::find_clickable_ids(child));
                }
            }
            VNode::Fragment(children) => {
                for child in children {
                    ids.extend(Self::find_clickable_ids(child));
                }
            }
            _ => {}
        }
        ids
    }
}
