use std::time::Duration;
use crossterm::event::{Event, KeyCode as CrossKeyCode, MouseEventKind, MouseButton as CrossMouseButton};
use crate::core::component::Component;
use crate::support::vector::Vec2;
use crate::platform::dispatcher::InputDispatcher;
use crate::platform::{SharedPlatformCore, runner::PlatformRunner, events::*};
use crate::renderer::tui::TuiRenderer;
use crate::renderer::Renderer;
use crate::support::error::RupauiError;
use super::terminal::TerminalInterface;

pub struct TerminalRunner {
    pub core: SharedPlatformCore,
    pub terminal: TerminalInterface,
    pub renderer: TuiRenderer,
    pub should_quit: bool,
}

impl TerminalRunner {
    pub fn new(core: SharedPlatformCore) -> Self {
        let terminal = TerminalInterface::new();
        let (cols, rows) = terminal.get_size();
        Self {
            core,
            terminal,
            renderer: TuiRenderer::new(cols, rows),
            should_quit: false,
        }
    }

    fn handle_redraw(&mut self) {
        let mut core = match self.core.write() {
            Ok(c) => c,
            Err(_) => return,
        };

        let scene_node = match core.compute_layout(&self.renderer, self.renderer.width() as f32, self.renderer.height() as f32) {
            Some(node) => node,
            None => return,
        };

        if let Some(ref root) = core.root {
            root.paint(
                &mut self.renderer,
                &core.scene.layout_engine.taffy,
                scene_node.raw(),
                false, 
                Vec2::zero(),
            );
        }
        self.renderer.present();
    }

    fn dispatch_event(&mut self, event: InputEvent) {
        let mut core = match self.core.write() {
            Ok(c) => c,
            Err(_) => return,
        };

        let mut cursor_pos = core.cursor_pos;
        let mut pointer_capture = core.pointer_capture.take();
        let mut focused_id = core.focused_id.take();
        let event_listeners = core.event_listeners.clone();
        let debug = core.debug;

        if let Some(ref root) = core.root {
            let root_ref: &dyn Component = root.as_ref();
            InputDispatcher::dispatch(
                event,
                root_ref,
                &core.scene,
                &mut cursor_pos,
                &mut pointer_capture,
                &mut focused_id,
                &event_listeners,
                debug,
            );
        }

        // Put things back
        core.cursor_pos = cursor_pos;
        core.pointer_capture = pointer_capture;
        core.focused_id = focused_id;
    }
}

impl PlatformRunner for TerminalRunner {
    fn run(mut self) -> Result<(), RupauiError> {
        self.terminal.setup().map_err(|e| RupauiError::Platform(e.to_string()))?;

        // For TUI, we don't have a winit event loop, but we could hook into the global proxy
        crate::platform::register_redraw_proxy(|| {
            // Placeholder for future event-driven TUI
        });

        while !self.should_quit {
            self.handle_redraw();

            if let Some(event) = self.terminal.poll_event(Duration::from_millis(16)).map_err(|e| RupauiError::Platform(e.to_string()))? {
                match event {
                    Event::Key(key) => {
                        let code = match key.code {
                            CrossKeyCode::Char(c) => KeyCode::Char(c),
                            CrossKeyCode::Enter => KeyCode::Enter,
                            CrossKeyCode::Esc => KeyCode::Escape,
                            CrossKeyCode::Tab => KeyCode::Tab,
                            CrossKeyCode::Backspace => KeyCode::Backspace,
                            CrossKeyCode::Up => KeyCode::ArrowUp,
                            CrossKeyCode::Down => KeyCode::ArrowDown,
                            CrossKeyCode::Left => KeyCode::ArrowLeft,
                            CrossKeyCode::Right => KeyCode::ArrowRight,
                            _ => KeyCode::Unknown,
                        };
                        
                        if let KeyCode::Char('q') = code { self.should_quit = true; }
                        
                        self.dispatch_event(InputEvent::Key { 
                            key: code, 
                            state: ButtonState::Pressed, 
                            modifiers: Modifiers::default() 
                        });
                    }
                    Event::Mouse(mouse) => {
                        let pos = Vec2::new(mouse.column as f32, mouse.row as f32);
                        self.dispatch_event(InputEvent::PointerMove { position: pos });

                        let button = match mouse.kind {
                            MouseEventKind::Down(btn) | MouseEventKind::Up(btn) | MouseEventKind::Drag(btn) => {
                                match btn {
                                    CrossMouseButton::Left => Some(PointerButton::Primary),
                                    CrossMouseButton::Right => Some(PointerButton::Secondary),
                                    CrossMouseButton::Middle => Some(PointerButton::Auxiliary),
                                }
                            }
                            _ => None,
                        };

                        if let Some(btn) = button {
                            let state = match mouse.kind {
                                MouseEventKind::Down(_) => Some(ButtonState::Pressed),
                                MouseEventKind::Up(_) => Some(ButtonState::Released),
                                _ => None,
                            };
                            if let Some(st) = state {
                                self.dispatch_event(InputEvent::PointerButton { button: btn, state: st });
                            }
                        }
                    }
                    Event::Resize(w, h) => {
                        self.renderer.resize(w, h);
                        let current_size = Vec2::new(w as f32, h as f32);
                        self.dispatch_event(InputEvent::Resize { 
                            size: current_size, 
                            scale_factor: 1.0 
                        });
                    }
                    _ => {}
                }
            }
        }

        self.terminal.restore().map_err(|e| RupauiError::Platform(e.to_string()))?;
        Ok(())
    }
}
