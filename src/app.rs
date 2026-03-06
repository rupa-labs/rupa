use crate::plugin::{PluginRegistry};
use crate::utils::{Theme, Vec2};
use crate::window::Window;
use crate::Component;
use crate::renderer::Renderer;
use winit::event_loop::{EventLoop, EventLoopProxy};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::window::WindowId;
use taffy::prelude::*;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug)]
pub enum RupauiEvent { RequestRedraw }
static EVENT_PROXY: OnceLock<EventLoopProxy<RupauiEvent>> = OnceLock::new();
pub fn request_redraw() { if let Some(proxy) = EVENT_PROXY.get() { let _ = proxy.send_event(RupauiEvent::RequestRedraw); } }

struct RupauiRunner {
    pub window: Option<Window>,
    pub renderer: Option<Renderer>,
    pub taffy: TaffyTree<()>,
    pub layout_map: HashMap<String, NodeId>,
    pub app_name: String,
    pub root: Option<Box<dyn Component>>,
    pub cursor_pos: Vec2,
    pub root_node: Option<NodeId>,
    pub hovered_node: Option<NodeId>,
}

impl RupauiRunner {
    fn is_node_hovered(&self, node: NodeId) -> bool {
        if let Ok(layout) = self.taffy.layout(node) {
            let x = layout.location.x; let y = layout.location.y;
            let w = layout.size.width; let h = layout.size.height;
            return self.cursor_pos.x >= x && self.cursor_pos.x <= x + w &&
                   self.cursor_pos.y >= y && self.cursor_pos.y <= y + h;
        }
        false
    }
    fn dispatch_click(&self, component: &dyn Component, node: NodeId) { if self.is_node_hovered(node) { component.on_click(); } }
    fn dispatch_scroll(&self, component: &dyn Component, node: NodeId, delta: f32) { if self.is_node_hovered(node) { component.on_scroll(delta); } }
    fn dispatch_drag(&self, component: &dyn Component, node: NodeId, delta: Vec2) { if self.is_node_hovered(node) { component.on_drag(delta); } }
}

impl ApplicationHandler<RupauiEvent> for RupauiRunner {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = Window::new(event_loop, &self.app_name, 1024, 768).unwrap();
            let renderer = pollster::block_on(Renderer::new(window.raw()));
            self.window = Some(window); self.renderer = Some(renderer);
        }
    }
    fn user_event(&mut self, _: &winit::event_loop::ActiveEventLoop, event: RupauiEvent) {
        match event { RupauiEvent::RequestRedraw => if let Some(ref window) = self.window { window.request_redraw(); } }
    }
    fn window_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => if let Some(ref mut r) = self.renderer { r.resize(size); },
            WindowEvent::CursorMoved { position, .. } => {
                let new_pos = Vec2::new(position.x as f32, position.y as f32);
                let delta = new_pos - self.cursor_pos;
                self.cursor_pos = new_pos;
                if let (Some(root), Some(node)) = (&self.root, self.root_node) { self.dispatch_drag(root.as_ref(), node, delta); }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll_y = match delta { winit::event::MouseScrollDelta::LineDelta(_, y) => y * 20.0, winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32 };
                if let (Some(root), Some(node)) = (&self.root, self.root_node) { self.dispatch_scroll(root.as_ref(), node, scroll_y); }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == winit::event::MouseButton::Left && state == winit::event::ElementState::Pressed {
                    if let (Some(root), Some(node)) = (&self.root, self.root_node) { self.dispatch_click(root.as_ref(), node); }
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(root), Some(renderer)) = (&self.root, &mut self.renderer) {
                    self.taffy.clear();
                    let root_node = root.layout(&mut self.taffy, None);
                    self.root_node = Some(root_node);
                    
                    let win_size = renderer.size;
                    self.taffy.compute_layout(root_node, Size { width: AvailableSpace::Definite(win_size.width as f32), height: AvailableSpace::Definite(win_size.height as f32) }).unwrap();

                    let is_hovered = {
                        let layout = self.taffy.layout(root_node).unwrap();
                        self.cursor_pos.x >= layout.location.x && self.cursor_pos.x <= layout.location.x + layout.size.width &&
                        self.cursor_pos.y >= layout.location.y && self.cursor_pos.y <= layout.location.y + layout.size.height
                    };

                    if let Ok((output, view, mut encoder)) = renderer.begin_frame() {
                        {
                            let mut _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
                                label: Some("Rupaui Pass"), 
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                                    view: &view, resolve_target: None, 
                                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store } 
                                })], 
                                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None 
                            });
                            root.paint(renderer, &self.taffy, root_node, is_hovered, &mut _pass);
                            renderer.flush_batches(&mut _pass);
                        }
                        renderer.end_frame(output, encoder);
                    }
                }
                if let Some(ref window) = self.window { window.request_redraw(); }
            }
            _ => (),
        }
    }
}

pub struct App { name: String, root: Option<Box<dyn Component>>, registry: PluginRegistry }
impl App {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), root: None, registry: PluginRegistry::new() } }
    pub fn root(mut self, component: impl Component + 'static) -> Self { self.root = Some(Box::new(component)); self }
    pub fn run(mut self) {
        let _ = Theme::current();
        let registry = std::mem::replace(&mut self.registry, PluginRegistry::new());
        registry.build_all(&mut self);
        let event_loop = winit::event_loop::EventLoop::<RupauiEvent>::with_user_event().build().unwrap();
        let _ = EVENT_PROXY.set(event_loop.create_proxy());
        let mut runner = RupauiRunner { window: None, renderer: None, taffy: TaffyTree::new(), layout_map: HashMap::new(), app_name: self.name.clone(), root: self.root, cursor_pos: Vec2::zero(), root_node: None, hovered_node: None };
        let _ = event_loop.run_app(&mut runner).unwrap();
    }
}
