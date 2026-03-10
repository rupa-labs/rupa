use rupa_core::{Component, Vec2, Error, Renderer};
use rupa_core::events::InputEvent;
use rupa_core::events::dispatcher::InputDispatcher;
use rupa_tui::TerminalRenderer;
use crate::platform::{SharedPlatformCore, runner::*, register_redraw_proxy, AppMetadata};
use crossterm::{
    event::{self, Event, KeyCode, MouseEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, size},
};
use std::time::Duration;

pub struct TerminalRunner {
    pub core: SharedPlatformCore,
    pub renderer: TerminalRenderer,
}

impl TerminalRunner {
    pub fn new(core: SharedPlatformCore) -> Self {
        let (w, h) = size().unwrap_or((80, 24));
        Self {
            core,
            renderer: TerminalRenderer::new(w as f32, h as f32),
        }
    }

    fn handle_redraw(&mut self) {
        let mut core = match self.core.write() {
            Ok(c) => c,
            Err(_) => return,
        };

        if let Some(root) = core.root.take() {
            let (w, h) = size().unwrap_or((80, 24));
            
            // 1. Build & Diff (Simplified for TUI MVP)
            let new_vnode = root.render();
            let old_vnode = root.get_prev_vnode();
            let _patches = rupa_core::reconciler::reconcile(&old_vnode, &new_vnode, None, 0);

            // 2. Compute layout
            let scene_node = core.scene.layout_engine.compute(
                root.as_ref(),
                &self.renderer,
                w as f32, 
                h as f32
            );

            // 3. Paint
            self.renderer.clear_screen();
            root.paint(
                &mut self.renderer,
                &core.scene.layout_engine.taffy,
                scene_node.raw(),
                false, 
                Vec2::zero(),
            );
            self.renderer.present();

            root.set_prev_vnode(new_vnode);
            core.root = Some(root);
        }
    }

    fn dispatch_event(&mut self, event: InputEvent) {
        let mut core = match self.core.write() {
            Ok(c) => c,
            Err(_) => return,
        };
        
        let mut cursor_pos = core.cursor_pos;
        let mut requested_cursor = core.requested_cursor;
        let mut pointer_capture = core.pointer_capture.take();
        let mut focused_id = core.focused_id.take();
        let mut hovered_path = std::mem::take(&mut core.hovered_path);
        let event_listeners = core.event_listeners.clone();
        
        if let Some(ref root) = core.root {
            InputDispatcher::dispatch(
                event,
                root.as_ref(),
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
        }

        core.cursor_pos = cursor_pos;
        core.requested_cursor = requested_cursor;
        core.pointer_capture = pointer_capture;
        core.focused_id = focused_id;
        core.hovered_path = hovered_path;
    }
}

impl PlatformRunner for TerminalRunner {
    fn sync_metadata(&self, _metadata: &AppMetadata) -> Result<(), Error> {
        Ok(())
    }

    fn run(mut self) -> Result<(), Error> {
        enable_raw_mode().map_err(|e| Error::Platform(format!("Failed to enable raw mode: {}", e)))?;
        
        register_redraw_proxy(Box::new(|| {}));

        loop {
            // 1. Tick animation
            if rupa_motion::GLOBAL_TIMELINE.tick() {
                // Animation running, redraw will happen below
            }

            // 2. Handle redraw
            self.handle_redraw();

            // 3. Handle Input
            if event::poll(Duration::from_millis(16)).unwrap_or(false) {
                match event::read().unwrap() {
                    Event::Key(key) => {
                        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                            break;
                        }
                        
                        let rupa_key = match key.code {
                            KeyCode::Enter => rupa_core::events::KeyCode::Enter,
                            KeyCode::Esc => rupa_core::events::KeyCode::Escape,
                            KeyCode::Up => rupa_core::events::KeyCode::ArrowUp,
                            KeyCode::Down => rupa_core::events::KeyCode::ArrowDown,
                            KeyCode::Left => rupa_core::events::KeyCode::ArrowLeft,
                            KeyCode::Right => rupa_core::events::KeyCode::ArrowRight,
                            KeyCode::Char(c) => rupa_core::events::KeyCode::Char(c),
                            _ => rupa_core::events::KeyCode::Unknown,
                        };

                        let state = rupa_core::events::ButtonState::Pressed;
                        let modifiers = rupa_core::events::Modifiers {
                            shift: key.modifiers.contains(KeyModifiers::SHIFT),
                            ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
                            alt: key.modifiers.contains(KeyModifiers::ALT),
                            logo: false,
                        };

                        self.dispatch_event(InputEvent::Key { 
                            key: rupa_key, 
                            state, 
                            modifiers 
                        });
                    }
                    Event::Resize(w, h) => {
                        self.renderer.core.logical_size = Vec2::new(w as f32, h as f32);
                    }
                    _ => {}
                }
            }
        }

        disable_raw_mode().unwrap();
        Ok(())
    }
}
