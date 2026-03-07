use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

use crate::platform::{SharedPlatformCore, runner::*, events::*, register_redraw_proxy};
use crate::renderer::gui::renderer::Renderer;
use crate::support::vector::Vec2;
use crate::support::error::RupauiError;

#[cfg(target_arch = "wasm32")]
use crate::platform::web::infra::WebInfra;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowAttributesExtWebSys;

/// The production-ready execution shell for Web browsers (WASM/Canvas).
pub struct WebRunner {
    pub core: SharedPlatformCore,
    pub canvas_id: String,
    pub window: Option<winit::window::Window>,
    pub renderer: Option<Renderer>,
}

impl WebRunner {
    pub fn new(core: SharedPlatformCore, canvas_id: impl Into<String>) -> Self {
        Self {
            core,
            canvas_id: canvas_id.into(),
            window: None,
            renderer: None,
        }
    }

    fn handle_redraw(&mut self) {
        let mut core = match self.core.write() {
            Ok(c) => c,
            Err(_) => return,
        };
        
        let renderer = match &mut self.renderer {
            Some(r) => r,
            None => return,
        };

        let (width, height) = renderer.core().logical_size.to_tuple();
        
        let scene_node = match core.compute_layout(renderer, width, height) {
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
}

impl PlatformRunner for WebRunner {
    fn run(self) -> Result<(), RupauiError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            return Err(RupauiError::Platform("WebRunner can only run on WASM32 targets".into()));
        }

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen_futures::spawn_local;

            let event_loop = EventLoop::<PlatformEvent>::with_user_event().build()
                .map_err(|e| RupauiError::Platform(e.to_string()))?;

            let proxy = event_loop.create_proxy();
            register_redraw_proxy(move || {
                let _ = proxy.send_event(PlatformEvent::RequestRedraw);
            });

            // On Web, the event loop should be started asynchronously.
            log::info!("Rupaui: Starting Web Event Loop...");
            event_loop.run_app(self).map_err(|e| RupauiError::Platform(e.to_string()))
        }
    }
}

impl ApplicationHandler<PlatformEvent> for WebRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(target_arch = "wasm32")]
        if self.window.is_none() {
            let canvas = match WebInfra::get_canvas(&self.canvas_id) {
                Ok(c) => c,
                Err(e) => {
                    log::error!("Failed to get canvas: {}", e);
                    return;
                }
            };

            WebInfra::fit_to_window(&canvas);

            let attributes = winit::window::WindowAttributes::default()
                .with_canvas(Some(canvas));
            
            match event_loop.create_window(attributes) {
                Ok(window) => {
                    let size = window.inner_size();
                    let scale = window.scale_factor();
                    
                    // Note: In a real WASM scenario, we need to handle the async Renderer initialization.
                    // This is a simplified version for high-performance WGPU on Web.
                    let renderer = pollster::block_on(Renderer::new(
                        std::sync::Arc::new(window.clone()), 
                        size.width, 
                        size.height, 
                        scale as f32
                    ));
                    
                    self.window = Some(window);
                    self.renderer = Some(renderer);
                }
                Err(e) => log::error!("Failed to create Web window: {}", e),
            }
        }
    }

    fn user_event(&mut self, _: &ActiveEventLoop, event: PlatformEvent) {
        if let PlatformEvent::RequestRedraw = event {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => self.handle_redraw(),
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size.width, size.height, self.window.as_ref().unwrap().scale_factor() as f32);
                }
            }
            // TODO: Map other browser events (Touch, Mouse, Key) to Rupaui InputEvent
            _ => {}
        }
    }
}
