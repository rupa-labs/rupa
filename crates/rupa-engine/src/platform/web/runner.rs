use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

use crate::platform::{SharedPlatformCore, runner::*, events::*, register_redraw_proxy};
use crate::renderer::gui::renderer::Renderer;
use rupa_core::vector::Vec2;
use rupa_core::error::Error;

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
    fn sync_metadata(&self, metadata: &AppMetadata) -> Result<(), Error> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    document.set_title(&metadata.title);
                }
            }

            if let Some(ref icon_source) = metadata.icon {
                if let Err(e) = crate::platform::web::infra::WebInfra::set_favicon(icon_source) {
                    log::error!("Web: Failed to set favicon: {}", e);
                }
            }

            if let Some(theme_color) = metadata.theme_color {
                if let Err(e) = crate::platform::web::infra::WebInfra::set_meta_theme_color(theme_color) {
                    log::error!("Web: Failed to set theme color meta: {}", e);
                }
            }
        }
        Ok(())
    }

    fn run(self) -> Result<(), Error> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            return Err(Error::Platform("WebRunner can only run on WASM32 targets".into()));
        }

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen_futures::spawn_local;

            // Sync initial path from URL
            let initial_path = WebInfra::get_current_path();
            crate::elements::routing::RouterState::push(initial_path);

            // Setup popstate listener
            let window = web_sys::window().unwrap();
            let closure = Closure::wrap(Box::new(move |_| {
                let path = WebInfra::get_current_path();
                crate::elements::routing::RouterState::push(path);
            }) as Box<dyn FnMut(web_sys::Event)>);
            window.add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget(); // Leak closure to keep it alive for the session

            let event_loop = EventLoop::<PlatformEvent>::with_user_event().build()
                .map_err(|e| Error::Platform(e.to_string()))?;

            let proxy = event_loop.create_proxy();
            register_redraw_proxy(move || {
                let _ = proxy.send_event(PlatformEvent::RequestRedraw);
            });

            // On Web, the event loop should be started asynchronously.
            log::info!("Rupa: Starting Web Event Loop...");
            event_loop.run_app(self).map_err(|e| Error::Platform(e.to_string()))
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
                &core.event_listeners,
                core.debug,
            );
        }

        // Map Rupa cursor to CSS
        if core.requested_cursor != requested_cursor {
            #[cfg(target_arch = "wasm32")]
            if let Some(window) = &self.window {
                let css_cursor = match requested_cursor {
                    CursorIcon::Default => "default",
                    CursorIcon::Pointer => "pointer",
                    CursorIcon::Text => "text",
                    CursorIcon::Grab => "grab",
                    CursorIcon::Grabbing => "grabbing",
                    CursorIcon::NotAllowed => "not-allowed",
                    CursorIcon::Wait => "wait",
                    CursorIcon::Crosshair => "crosshair",
                };
                use winit::platform::web::WindowExtWebSys;
                if let Some(canvas) = window.canvas() {
                    let _ = canvas.style().set_property("cursor", css_cursor);
                }
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
                    let core_lock = self.core.read().unwrap();
                    let _ = self.sync_metadata(&core_lock.metadata);
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
            _ => {
                // log::trace!("Web: Unhandled browser event: {:?}", event);
            }
        }
    }
}
