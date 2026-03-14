use rupa_core::{Vec2, Error, Renderer, vnode::VNode};
use rupa_core::events::InputEvent;
use rupa_core::events::dispatcher::InputDispatcher;
use crate::TerminalRenderer;
use rupa_engine::platform::{SharedPlatformCore, runner::*, register_redraw_proxy, AppMetadata};
use crossterm::{
    event::{self, Event, KeyCode, MouseEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show},
    ExecutableCommand,
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
        let core = match self.core.read() {
            Ok(c) => c,
            Err(_) => return,
        };

        if let Some(root) = core.element_tree.root() {
            let (w, h) = size().unwrap_or((80, 24));
            
            // 1. Build VNode Tree
            let vnode = root.render();

            // 2. Compute Layout (using the decoupled engine)
            let mut write_core = self.core.write().unwrap();
            let scene_node = write_core.scene.layout_engine.compute(
                &vnode, 
                &self.renderer, 
                w as f32, 
                h as f32
            );
            write_core.scene.set_root(scene_node);
            drop(write_core);

            // 3. Paint (Refactored to be agnostic)
            self.renderer.clear_screen();
            // In a full implementation, we would walk the VNode tree and call renderer methods
            // self.renderer.paint_vnode(&vnode, ...);
            self.renderer.present();
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
        
        if let Some(root) = core.element_tree.root() {
            InputDispatcher::dispatch(
                event,
                &root.render(), // Dispatch into the rendered VNode tree
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
        let mut out = std::io::stdout();
        enable_raw_mode().map_err(|e| Error::Platform(format!("Failed to enable raw mode: {}", e)))?;
        out.execute(EnterAlternateScreen).unwrap();
        out.execute(Hide).unwrap();
        
        register_redraw_proxy(Box::new(|| {}));

        loop {
            if rupa_motion::GLOBAL_TIMELINE.tick() { }
            self.handle_redraw();

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

        out.execute(Show).unwrap();
        out.execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        Ok(())
    }
}
