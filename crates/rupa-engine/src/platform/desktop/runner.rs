use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{WindowId, Window as WinitWindow};
use std::sync::Arc;

use rupa_core::{Component, Renderer};
use crate::renderer::gui::renderer::Renderer as GuiRenderer;
use rupa_core::{Vec2, Error, Signal, Readable, generate_id, CursorIcon};
use rupa_core::events::{InputEvent, UIEvent, EventListeners, Modifiers, PointerButton, ButtonState, KeyCode};
use crate::platform::dispatcher::InputDispatcher;
use crate::platform::desktop::input::map_key;
use crate::platform::{SharedPlatformCore, runner::*, register_redraw_proxy, AppMetadata};

pub struct DesktopRunner {
    pub core: SharedPlatformCore,
    pub window: Option<Arc<WinitWindow>>,
    pub renderer: Option<GuiRenderer>,
    pub scale_factor: f64,
    pub modifiers: Modifiers,
}

impl DesktopRunner {
    pub fn new(core: SharedPlatformCore) -> Self {
        Self {
            core,
            window: None,
            renderer: None,
            scale_factor: 1.0,
            modifiers: Modifiers::default(),
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

        if let Some(root) = core.root.take() {
            // Compute layout
            let scene_node = core.scene.layout_engine.compute(
                root.as_ref(),
                renderer,
                win_width as f32 / self.scale_factor as f32, 
                win_height as f32 / self.scale_factor as f32
            );

            if let Ok(()) = renderer.begin_frame() {
                root.paint(
                    renderer,
                    &core.scene.layout_engine.taffy,
                    scene_node.raw(),
                    false, 
                    Vec2::zero(),
                );
                renderer.present();
            }
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
                &event_listeners,
                core.debug,
            );
        }

        // Handle Cursor change request
        if core.requested_cursor != requested_cursor {
            if let Some(window) = &self.window {
                let winit_cursor = match requested_cursor {
                    CursorIcon::Default => winit::window::CursorIcon::Default,
                    CursorIcon::Pointer => winit::window::CursorIcon::Pointer,
                    CursorIcon::Text => winit::window::CursorIcon::Text,
                    CursorIcon::Grab => winit::window::CursorIcon::Grab,
                    CursorIcon::Grabbing => winit::window::CursorIcon::Grabbing,
                    CursorIcon::NotAllowed => winit::window::CursorIcon::NotAllowed,
                    CursorIcon::Wait => winit::window::CursorIcon::Wait,
                    CursorIcon::Crosshair => winit::window::CursorIcon::Crosshair,
                };
                window.set_cursor(winit_cursor);
            }
        }

        core.cursor_pos = cursor_pos;
        core.requested_cursor = requested_cursor;
        core.pointer_capture = pointer_capture;
        core.focused_id = focused_id;
        
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

impl PlatformRunner for DesktopRunner {
    fn sync_metadata(&self, metadata: &AppMetadata) -> Result<(), Error> {
        if let Some(window) = &self.window {
            window.set_title(&metadata.title);
        }
        Ok(())
    }

    fn run(mut self) -> Result<(), Error> {
        let event_loop = EventLoop::<PlatformEvent>::with_user_event().build()
            .map_err(|e| Error::Platform(format!("Failed to build event loop: {}", e)))?;
            
        let proxy = event_loop.create_proxy();
        register_redraw_proxy(Box::new(move || {
            let _ = proxy.send_event(PlatformEvent::RequestRedraw);
        }));

        event_loop.run_app(&mut self)
            .map_err(|e| Error::Platform(format!("Failed to run app: {}", e)))
    }
}

impl ApplicationHandler<PlatformEvent> for DesktopRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let core_lock = self.core.read().unwrap();
            let title = core_lock.metadata.title.clone();
            
            let window_attributes = WinitWindow::default_attributes()
                .with_title(title)
                .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0));
            
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let size = window.inner_size();
            let scale = window.scale_factor();
            let renderer = pollster::block_on(GuiRenderer::new(window.clone(), size.width, size.height, scale as f32));
            
            self.scale_factor = scale;
            self.window = Some(window);
            self.renderer = Some(renderer);
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
            }
            WindowEvent::CursorMoved { position, .. } => {
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
                self.dispatch_event(InputEvent::Key { 
                    key: key_code, 
                    state, 
                    modifiers: self.modifiers 
                });
            }
            WindowEvent::RedrawRequested => {
                self.handle_redraw();
            }
            _ => {}
        }
    }
}
