use rupa_core::{Vec2, Error, VNode, Renderer, reconciler::reconcile};
use rupa_core::events::InputEvent;
use rupa_tui::TerminalRenderer;
use rupa_engine::platform::{SharedPlatformCore, runner::*, register_redraw_proxy, AppMetadata};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show},
    ExecutableCommand,
};
use std::time::Duration;
use taffy::prelude::*;

pub mod layout;
pub mod paint;
pub mod events;

use layout::LayoutEngine;
use paint::Painter;
use events::EventHandler;

pub struct TerminalRunner {
    pub core: SharedPlatformCore,
    pub renderer: TerminalRenderer,
    pub last_vnode: VNode,
}

impl TerminalRunner {
    pub fn new(core: SharedPlatformCore) -> Self {
        let (w, h) = size().unwrap_or((80, 24));
        Self {
            core,
            renderer: TerminalRenderer::new(w as f32, h as f32),
            last_vnode: VNode::Empty,
        }
    }

    fn handle_redraw(&mut self) {
        let mut core = match self.core.write() {
            Ok(c) => c,
            Err(_) => return,
        };

        if let Some(root) = core.root.take() {
            let (w, h) = size().unwrap_or((80, 24));
            
            // 1. Render Current Tree
            let new_vnode = root.render();

            // 2. Reconcile (Identify Changes)
            let _patches = reconcile(&self.last_vnode, &new_vnode, None, 0);
            
            // 3. Update Taffy Tree (For Layout & Hit Testing)
            core.scene.layout_engine.taffy.clear();
            let layout_root = LayoutEngine::build_taffy_from_vnode(&new_vnode, &mut core.scene.layout_engine.taffy);
            
            core.scene.layout_engine.taffy.compute_layout(
                layout_root, 
                Size { width: AvailableSpace::Definite(w as f32), height: AvailableSpace::Definite(h as f32) }
            ).unwrap();

            // 4. Paint the VNode tree
            self.renderer.clear_screen();
            let focused_id = core.focused_id.clone();
            Painter::paint_vnode(
                &mut self.renderer, 
                &new_vnode, 
                &core.scene.layout_engine.taffy, 
                layout_root, 
                Vec2::zero(), 
                focused_id.as_deref(), 
                rupa_core::renderer::TypographyStyle::default()
            );
            self.renderer.present();

            self.last_vnode = new_vnode;
            core.root = Some(root);
        }
    }
}

impl PlatformRunner for TerminalRunner {
    fn sync_metadata(&self, _metadata: &AppMetadata) -> Result<(), Error> {
        Ok(())
    }

    fn run(mut self) -> Result<(), Error> {
        let mut out = std::io::stdout();
        
        // 1. Initial State Setup
        enable_raw_mode().map_err(|e| Error::Platform(format!("Failed to enable raw mode: {}", e)))?;
        out.execute(EnterAlternateScreen).unwrap();
        out.execute(Hide).unwrap();
        
        register_redraw_proxy(Box::new(|| {}));

        // 2. Main Loop
        let res = (|| {
            loop {
                if rupa_motion::GLOBAL_TIMELINE.tick() { }
                self.handle_redraw();

                if event::poll(Duration::from_millis(32)).unwrap_or(false) {
                    match event::read().unwrap() {
                        Event::Key(key) => {
                            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                                return Ok(());
                            }
                            
                            let rupa_key = match key.code {
                                KeyCode::Enter => rupa_core::events::KeyCode::Enter,
                                KeyCode::Esc => rupa_core::events::KeyCode::Escape,
                                KeyCode::Up => rupa_core::events::KeyCode::ArrowUp,
                                KeyCode::Down => rupa_core::events::KeyCode::ArrowDown,
                                KeyCode::Left => rupa_core::events::KeyCode::ArrowLeft,
                                KeyCode::Right => rupa_core::events::KeyCode::ArrowRight,
                                KeyCode::Char(c) => rupa_core::events::KeyCode::Char(c),
                                KeyCode::Tab => rupa_core::events::KeyCode::Tab,
                                KeyCode::Backspace => rupa_core::events::KeyCode::Backspace,
                                _ => rupa_core::events::KeyCode::Unknown,
                            };

                            let state = rupa_core::events::ButtonState::Pressed;
                            let modifiers = rupa_core::events::Modifiers {
                                shift: key.modifiers.contains(KeyModifiers::SHIFT),
                                ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
                                alt: key.modifiers.contains(KeyModifiers::ALT),
                                logo: false,
                            };

                            EventHandler::dispatch(&self.core, InputEvent::Key { 
                                key: rupa_key, 
                                state, 
                                modifiers 
                            }, &self.last_vnode);
                        }
                        Event::Resize(w, h) => {
                            self.renderer.resize(w as f32, h as f32);
                        }
                        _ => {}
                    }
                }
            }
        })();

        // 3. Guaranteed Cleanup Logic
        out.execute(Show).unwrap();
        out.execute(LeaveAlternateScreen).unwrap();
        let _ = disable_raw_mode();
        
        res
    }
}
