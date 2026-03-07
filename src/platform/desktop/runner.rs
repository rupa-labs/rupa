use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{WindowId, Window as WinitWindow};
use std::sync::Arc;

use crate::core::component::Component;
use crate::renderer::Renderer as _;
use crate::renderer::gui::renderer::Renderer;
use crate::support::vector::Vec2;
use crate::platform::dispatcher::InputDispatcher;
use crate::platform::desktop::input::map_key;
use crate::platform::{SharedPlatformCore, runner::*, events::*, register_redraw_proxy};

pub struct DesktopRunner {
    pub core: SharedPlatformCore,
    pub window: Option<Arc<WinitWindow>>,
    pub renderer: Option<Renderer>,
    pub scale_factor: f64,
    pub modifiers: Modifiers,
    pub a11y_adapter: Option<Box<dyn std::any::Any + Send + Sync>>,
}

impl DesktopRunner {
    pub fn new(core: SharedPlatformCore) -> Self {
        Self {
            core,
            window: None,
            renderer: None,
            scale_factor: 1.0,
            modifiers: Modifiers::default(),
            a11y_adapter: None,
        }
    }

    fn handle_redraw(&mut self) {
        let mut core = match self.core.write() {
            Ok(c) => c,
            Err(_) => return,
        };
        
        let (win_width, win_height) = if let Some(window) = &self.window {
            let size = window.inner_size();
            (size.width, size.height)
        } else {
            return;
        };
        
        let renderer = match &mut self.renderer {
            Some(r) => r,
            None => return,
        };

        // Compute layout in logical units
        let scene_node = match core.compute_layout(
            renderer,
            win_width as f32 / self.scale_factor as f32, 
            win_height as f32 / self.scale_factor as f32
        ) {
            Some(node) => node,
            None => return,
        };

        if let Ok(()) = renderer.begin_frame() {
            if let Some(ref root) = core.root {
                root.paint(
                    renderer,
                    &core.scene.layout_engine.taffy,
                    scene_node.raw(),
                    false, 
                    Vec2::zero(),
                );
            }
            renderer.present();
        }
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
                core.debug,
            );
        }

        // Put things back
        core.cursor_pos = cursor_pos;
        core.pointer_capture = pointer_capture;
        core.focused_id = focused_id;
        
        // Request redraw after dispatching user input as it likely changed something visual
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

impl PlatformRunner for DesktopRunner {
    fn run(mut self) -> Result<(), crate::support::error::RupauiError> {
        let event_loop = EventLoop::<PlatformEvent>::with_user_event().build()
            .map_err(|e| crate::support::error::RupauiError::Platform(format!("Failed to build event loop: {}", e)))?;
            
        let proxy = event_loop.create_proxy();
        register_redraw_proxy(move || {
            let _ = proxy.send_event(PlatformEvent::RequestRedraw);
        });

        event_loop.run_app(&mut self)
            .map_err(|e| crate::support::error::RupauiError::Platform(format!("Failed to run app: {}", e)))
    }
}

impl ApplicationHandler<PlatformEvent> for DesktopRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let core_lock = match self.core.read() {
                Ok(c) => c,
                Err(e) => {
                    log::error!("Failed to acquire core lock: {}", e);
                    event_loop.exit();
                    return;
                }
            };

            let title = core_lock.metadata.title.clone();
            match crate::platform::desktop::infra::DesktopInfra::create_window(event_loop, &title, 1024, 768) {
                Ok(window) => {
                    let size = window.inner_size();
                    let scale = window.scale_factor();
                    let renderer = pollster::block_on(Renderer::new(window.clone(), size.width, size.height, scale as f32));
                    self.scale_factor = scale;
                    self.window = Some(window);
                    self.renderer = Some(renderer);
                }
                Err(e) => {
                    log::error!("Failed to create window: {}", e);
                    event_loop.exit();
                }
            }
        }
    }

    fn user_event(&mut self, _: &ActiveEventLoop, event: PlatformEvent) {
        let PlatformEvent::RequestRedraw = event;
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.dispatch_event(InputEvent::Quit);
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                let scale = self.scale_factor;
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size.width, size.height, scale as f32);
                }
                self.dispatch_event(InputEvent::Resize { 
                    size: Vec2::new(size.width as f32 / scale as f32, size.height as f32 / scale as f32),
                    scale_factor: scale 
                });
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = scale_factor;
                if let Some(window) = &self.window {
                    let size = window.inner_size();
                    if let Some(renderer) = &mut self.renderer {
                        renderer.resize(size.width, size.height, scale_factor as f32);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                // Convert physical pixel position to logical position
                let logical_pos = Vec2::new(
                    (position.x / self.scale_factor) as f32, 
                    (position.y / self.scale_factor) as f32
                );
                self.dispatch_event(InputEvent::PointerMove { position: logical_pos });
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                let state = modifiers.state();
                self.modifiers = Modifiers {
                    shift: state.shift_key(),
                    ctrl: state.control_key(),
                    alt: state.alt_key(),
                    logo: state.super_key(),
                };
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let btn = match button {
                    MouseButton::Left => PointerButton::Primary,
                    MouseButton::Right => PointerButton::Secondary,
                    MouseButton::Middle => PointerButton::Auxiliary,
                    MouseButton::Back => PointerButton::Extra(0),
                    MouseButton::Forward => PointerButton::Extra(1),
                    MouseButton::Other(id) => PointerButton::Extra(id),
                };
                let st = match state {
                    ElementState::Pressed => ButtonState::Pressed,
                    ElementState::Released => ButtonState::Released,
                };
                self.dispatch_event(InputEvent::PointerButton { button: btn, state: st });
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll = match delta {
                    MouseScrollDelta::LineDelta(x, y) => Vec2::new(x, y * 20.0),
                    MouseScrollDelta::PixelDelta(pos) => Vec2::new(
                        (pos.x / self.scale_factor) as f32, 
                        (pos.y / self.scale_factor) as f32
                    ),
                };
                self.dispatch_event(InputEvent::PointerScroll { delta: scroll });
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let key_code = map_key(&event);
                let state = match event.state {
                    ElementState::Pressed => ButtonState::Pressed,
                    ElementState::Released => ButtonState::Released,
                };

                // Exit on Q (keep as a convenience for now)
                if state == ButtonState::Pressed && key_code == KeyCode::Char('Q') {
                     event_loop.exit();
                }

                self.dispatch_event(InputEvent::Key { 
                    key: key_code, 
                    state, 
                    modifiers: self.modifiers 
                });
            }
            WindowEvent::Ime(winit::event::Ime::Commit(text)) => {
                self.dispatch_event(InputEvent::Ime(text));
            }
            WindowEvent::RedrawRequested => {
                self.handle_redraw();
            }
            _ => {}
        }
    }
}
