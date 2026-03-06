pub mod events;
pub mod window;
pub mod dispatcher;

use std::sync::OnceLock;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::window::WindowId;

use crate::core::component::Component;
use crate::core::plugin::PluginRegistry;
use crate::renderer::renderer::Renderer;
use crate::layout::engine::LayoutEngine;
use crate::utils::vector::Vec2;
use crate::utils::Theme;
use crate::platform::window::Window;
use crate::platform::dispatcher::EventDispatcher;

#[derive(Debug)]
pub enum RupauiEvent {
    RequestRedraw,
}

static EVENT_PROXY: OnceLock<EventLoopProxy<RupauiEvent>> = OnceLock::new();

pub fn request_redraw() {
    if let Some(proxy) = EVENT_PROXY.get() {
        let _ = proxy.send_event(RupauiEvent::RequestRedraw);
    }
}

pub struct RupauiRunner {
    pub window: Option<Window>,
    pub renderer: Option<Renderer>,
    pub layout_engine: LayoutEngine,

    pub app_name: String,
    pub root: Option<Box<dyn Component>>,

    pub cursor_pos: Vec2,
    pub root_node: Option<taffy::prelude::NodeId>,
}

impl RupauiRunner {
    pub fn new(app_name: String, root: Option<Box<dyn Component>>) -> Self {
        Self {
            window: None,
            renderer: None,
            layout_engine: LayoutEngine::new(),
            app_name,
            root,
            cursor_pos: Vec2::zero(),
            root_node: None,
        }
    }

    fn handle_redraw(&mut self) {
        let root = match &self.root {
            Some(r) => r,
            None => return,
        };

        let (win_width, win_height) = self.window.as_ref().unwrap().size();
        let root_node = self.layout_engine.compute(root.as_ref(), win_width as f32, win_height as f32);
        self.root_node = Some(root_node);

        let renderer = match &mut self.renderer {
            Some(r) => r,
            None => return,
        };

        if let Ok((output, view, mut encoder)) = renderer.begin_frame() {
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Rupaui UI Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None, depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None, multiview_mask: None,
                });

                root.paint(
                    renderer,
                    &self.layout_engine.taffy,
                    root_node,
                    false, 
                    &mut pass,
                    Vec2::zero(),
                );

                renderer.flush_batches(&mut pass);
            }
            renderer.end_frame(output, encoder);
        }
    }
}

impl ApplicationHandler<RupauiEvent> for RupauiRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Window::new(event_loop, &self.app_name, 1024, 768).unwrap();
            let renderer = pollster::block_on(Renderer::new(window.raw()));

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
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let new_pos = Vec2::new(position.x as f32, position.y as f32);
                let delta = new_pos - self.cursor_pos;
                self.cursor_pos = new_pos;
                
                if let (Some(root), Some(node)) = (&self.root, self.root_node) {
                    if let Some(hit) = EventDispatcher::hit_test(&self.layout_engine.taffy, root.as_ref(), node, self.cursor_pos, Vec2::zero()) {
                        EventDispatcher::dispatch_drag(hit, delta);
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll_y = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y * 20.0,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };
                if let (Some(root), Some(node)) = (&self.root, self.root_node) {
                    if let Some(hit) = EventDispatcher::hit_test(&self.layout_engine.taffy, root.as_ref(), node, self.cursor_pos, Vec2::zero()) {
                        EventDispatcher::dispatch_scroll(hit, scroll_y);
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    if let (Some(root), Some(node)) = (&self.root, self.root_node) {
                        if let Some(hit) = EventDispatcher::hit_test(&self.layout_engine.taffy, root.as_ref(), node, self.cursor_pos, Vec2::zero()) {
                            EventDispatcher::dispatch_click(hit);
                        }
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

pub struct App {
    pub name: String,
    pub root: Option<Box<dyn Component>>,
    pub registry: PluginRegistry,
}

impl App {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            root: None,
            registry: PluginRegistry::new(),
        }
    }

    pub fn root(mut self, component: impl Component + 'static) -> Self {
        self.root = Some(Box::new(component));
        self
    }

    pub fn run(mut self) {
        let _ = Theme::current();
        let registry = std::mem::replace(&mut self.registry, PluginRegistry::new());
        registry.build_all(&mut self);

        let event_loop = winit::event_loop::EventLoop::<RupauiEvent>::with_user_event()
            .build()
            .unwrap();

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        let _ = EVENT_PROXY.set(event_loop.create_proxy());

        let mut runner = RupauiRunner::new(self.name.clone(), self.root);
        event_loop.run_app(&mut runner).unwrap();
    }
}
