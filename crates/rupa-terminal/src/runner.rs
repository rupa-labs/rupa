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

pub mod layout;
pub mod paint;
pub mod events;

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
        let core_read = match self.core.read() {
            Ok(c) => c,
            Err(_) => return,
        };

        if let Some(root) = core_read.element_tree.root() {
            let (w, h) = size().unwrap_or((80, 24));
            
            // 1. Render Current Tree
            let new_vnode = root.render();

            // 2. Reconcile (Identify Changes)
            let _patches = reconcile(&self.last_vnode, &new_vnode, None, 0);
            
            // 3. Update Scene & Layout
            let mut core_write = self.core.write().unwrap();
            let scene_node = core_write.scene.layout_engine.compute(
                &new_vnode, 
                &self.renderer, 
                w as f32, 
                h as f32
            );
            core_write.scene.set_root(scene_node);
            
            let focused_id = core_write.focused_id.clone();
            let layout_engine = &core_write.scene.layout_engine;

            // 4. Paint the VNode tree
            self.renderer.clear_screen();
            Painter::paint_vnode(
                &mut self.renderer, 
                &new_vnode, 
                layout_engine,
                scene_node, 
                Vec2::zero(), 
                focused_id.as_deref(), 
                rupa_core::renderer::TypographyStyle::default()
            );
            self.renderer.present();

            self.last_vnode = new_vnode;
        }
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

        out.execute(Show).unwrap();
        out.execute(LeaveAlternateScreen).unwrap();
        let _ = disable_raw_mode();
        
        res
    }
}
