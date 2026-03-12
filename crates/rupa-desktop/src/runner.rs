use rupa_core::{Error, renderer::Renderer as BaseRenderer};
use rupa_engine::platform::{context::SharedPlatformCore, runner::PlatformRunner, app::AppMetadata};
use crate::renderer::renderer::Renderer;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, ControlFlow};
use winit::window::{Window, WindowId};

pub struct DesktopRunner {
    pub core: SharedPlatformCore,
    pub window: Option<Arc<Window>>,
    pub renderer: Option<Renderer>,
}

impl DesktopRunner {
    pub fn new(core: SharedPlatformCore) -> Self {
        Self {
            core,
            window: None,
            renderer: None,
        }
    }

    fn handle_redraw(&mut self) {
        if let Some(ref mut renderer) = self.renderer {
            if let Ok(_) = renderer.begin_frame() {
                // In a real implementation, we would traverse the element tree
                // and call draw_rect/draw_text based on the scene graph.
                
                renderer.present();
            }
        }
    }
}

impl ApplicationHandler for DesktopRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Rupa Desktop");
            
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            self.window = Some(window.clone());

            // Initialize GPU Renderer asynchronously
            let width = window.inner_size().width;
            let height = window.inner_size().height;
            let scale_factor = window.scale_factor() as f32;
            
            // Note: Since resumed is synchronous but Renderer::new is async,
            // we use pollster for initialization in this runner.
            let renderer = pollster::block_on(Renderer::new(window, width, height, scale_factor));
            self.renderer = Some(renderer);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let (Some(ref mut renderer), Some(ref window)) = (&mut self.renderer, &self.window) {
                    renderer.resize(size.width, size.height, window.scale_factor() as f32);
                }
            }
            WindowEvent::RedrawRequested => {
                self.handle_redraw();
                
                // Request next frame if animations are active
                if rupa_motion::GLOBAL_TIMELINE.tick() {
                    if let Some(ref win) = self.window {
                        win.request_redraw();
                    }
                }
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // High-frequency tick for animations if needed outside redraw
    }
}

impl PlatformRunner for DesktopRunner {
    fn sync_metadata(&self, _metadata: &AppMetadata) -> Result<(), Error> {
        Ok(())
    }

    fn run(mut self) -> Result<(), Error> {
        let event_loop = EventLoop::new().map_err(|e| Error::Platform(format!("EventLoop error: {}", e)))?;
        event_loop.set_control_flow(ControlFlow::Wait);
        
        event_loop.run_app(&mut self).map_err(|e| Error::Platform(format!("Run error: {}", e)))?;
        
        Ok(())
    }
}
