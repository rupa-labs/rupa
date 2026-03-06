pub mod terminal;

use std::time::Duration;
use crossterm::event::{Event, KeyCode as CrossKeyCode, MouseEventKind, MouseButton as CrossMouseButton};
use crate::core::component::Component;
use crate::support::vector::Vec2;
use crate::platform::dispatcher::InputDispatcher;
use crate::platform::events::*;
use crate::platform::PlatformCore;
use crate::renderer::tui::TuiRenderer;
use crate::renderer::Renderer;
use self::terminal::TerminalInterface;

pub struct TuiRunner {
    pub core: PlatformCore,
    pub terminal: TerminalInterface,
    pub renderer: TuiRenderer,
    pub should_quit: bool,
}

impl TuiRunner {
    pub fn new(app_name: String, root: Option<Box<dyn Component>>) -> Self {
        let terminal = TerminalInterface::new();
        let (cols, rows) = terminal.get_size();
        Self {
            core: PlatformCore::new(app_name, root),
            terminal,
            renderer: TuiRenderer::new(cols, rows),
            should_quit: false,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.setup()?;

        let (cols, rows) = self.terminal.get_size();
        let mut current_size = Vec2::new(cols as f32, rows as f32);

        while !self.should_quit {
            self.handle_redraw();

            if let Some(event) = self.terminal.poll_event(Duration::from_millis(16))? {
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

                        let button = match mouse.button {
                            CrossMouseButton::Left => Some(PointerButton::Primary),
                            CrossMouseButton::Right => Some(PointerButton::Secondary),
                            CrossMouseButton::Middle => Some(PointerButton::Auxiliary),
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
                        current_size = Vec2::new(w as f32, h as f32);
                        self.dispatch_event(InputEvent::Resize { 
                            size: current_size, 
                            scale_factor: 1.0 
                        });
                    }
                    _ => {}
                }
            }
        }

        self.terminal.restore()?;
        Ok(())
    }

    fn handle_redraw(&mut self) {
        let scene_node = self.core.compute_layout(self.renderer.width() as f32, self.renderer.height() as f32);

        if let Some(ref root) = self.core.root {
            root.paint(
                &mut self.renderer,
                &self.core.scene.layout_engine.taffy,
                scene_node.raw(),
                false, 
                Vec2::zero(),
            );
        }
        self.renderer.present();
    }

    fn dispatch_event(&mut self, event: InputEvent) {
        if let Some(ref root) = self.core.root {
            InputDispatcher::dispatch(
                event,
                root.as_ref(),
                &self.core.scene,
                &mut self.core.cursor_pos,
            );
        }
    }
}
