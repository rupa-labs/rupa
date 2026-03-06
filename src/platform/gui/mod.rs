pub mod window;

use std::sync::OnceLock;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent, KeyEvent};
use winit::event_loop::{ActiveEventLoop, EventLoopProxy, EventLoop};
use winit::window::WindowId;
use winit::keyboard::{PhysicalKey, KeyCode as WinitKeyCode};

use crate::core::component::Component;
use crate::renderer::Renderer as _;
use crate::renderer::gui::renderer::Renderer;
use crate::support::vector::Vec2;
use crate::platform::gui::window::Window;
use crate::platform::dispatcher::InputDispatcher;
use crate::platform::{RupauiEvent, PlatformCore, events::*};

static EVENT_PROXY: OnceLock<EventLoopProxy<RupauiEvent>> = OnceLock::new();

pub fn get_event_proxy() -> Option<&'static EventLoopProxy<RupauiEvent>> {
    EVENT_PROXY.get()
}

pub fn set_event_proxy(proxy: EventLoopProxy<RupauiEvent>) {
    let _ = EVENT_PROXY.set(proxy);
}

pub struct GuiRunner {
    pub core: PlatformCore,
    pub window: Option<Window>,
    pub renderer: Option<Renderer>,
    pub scale_factor: f64,
}

impl GuiRunner {
    pub fn new(app_name: String, root: Option<Box<dyn Component>>) -> Self {
        Self {
            core: PlatformCore::new(app_name, root),
            window: None,
            renderer: None,
            scale_factor: 1.0,
        }
    }

    pub fn run_app(mut self) {
        let event_loop = EventLoop::<RupauiEvent>::with_user_event().build().unwrap();
        set_event_proxy(event_loop.create_proxy());
        event_loop.run_app(&mut self).unwrap();
    }

    fn handle_redraw(&mut self) {
        let (win_width, win_height) = self.window.as_ref().unwrap().size();
        let scene_node = self.core.compute_layout(win_width as f32, win_height as f32);

        let renderer = match &mut self.renderer {
            Some(r) => r,
            None => return,
        };

        if let Ok(()) = renderer.begin_frame() {
            if let Some(ref root) = self.core.root {
                root.paint(
                    renderer,
                    &self.core.scene.layout_engine.taffy,
                    scene_node.raw(),
                    false, 
                    Vec2::zero(),
                );
            }
            renderer.present();
        }
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

impl ApplicationHandler<RupauiEvent> for GuiRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Window::new(event_loop, &self.core.app_name, 1024, 768).unwrap();
            let renderer = pollster::block_on(Renderer::new(window.raw()));
            self.scale_factor = window.raw().scale_factor();
            self.window = Some(window);
            self.renderer = Some(renderer);
        }
    }

    fn user_event(&mut self, _: &ActiveEventLoop, event: RupauiEvent) {
        let RupauiEvent::RequestRedraw = event;
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
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size);
                }
                let scale = self.scale_factor;
                self.dispatch_event(InputEvent::Resize { 
                    size: Vec2::new(size.width as f32, size.height as f32),
                    scale_factor: scale 
                });
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = scale_factor;
            }
            WindowEvent::CursorMoved { position, .. } => {
                let logical_pos = Vec2::new(position.x as f32, position.y as f32);
                self.dispatch_event(InputEvent::PointerMove { position: logical_pos });
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let btn = match button {
                    MouseButton::Left => PointerButton::Primary,
                    MouseButton::Right => PointerButton::Secondary,
                    MouseButton::Middle => PointerButton::Auxiliary,
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
                    MouseScrollDelta::PixelDelta(pos) => Vec2::new(pos.x as f32, pos.y as f32),
                };
                self.dispatch_event(InputEvent::PointerScroll { delta: scroll });
            }
            WindowEvent::KeyboardInput { event: KeyEvent { physical_key, state, .. }, .. } => {
                if state == ElementState::Pressed {
                    if let PhysicalKey::Code(WinitKeyCode::KeyQ) = physical_key {
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.handle_redraw();
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}
