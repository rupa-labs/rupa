use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

use crate::platform::{SharedPlatformCore, runner::*, events::*, register_redraw_proxy};
use crate::renderer::gui::renderer::Renderer;
use rupa_core::vector::Vec2;
use rupa_core::error::Error;

/// The production-ready execution shell for Mobile platforms (Android/iOS).
/// Handles complex mobile lifecycles (Suspend/Resume/Low Memory).
pub struct MobileRunner {
    pub core: SharedPlatformCore,
    pub window: Option<winit::window::Window>,
    pub renderer: Option<Renderer>,
}

impl MobileRunner {
    pub fn new(core: SharedPlatformCore) -> Self {
        Self {
            core,
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

impl PlatformRunner for MobileRunner {
    fn sync_metadata(&self, metadata: &AppMetadata) -> Result<(), Error> {
        if let Some(window) = &self.window {
            window.set_title(&metadata.title);
            
            if let Some(ref _icon) = metadata.icon {
                log::warn!("Mobile: Dynamic icon sync with task switcher is currently unsupported.");
            }

            if let Some(_color) = metadata.theme_color {
                log::warn!("Mobile: Status bar color synchronization is currently unsupported.");
            }
        }
        Ok(())
    }

    fn run(self) -> Result<(), Error> {
        let event_loop = EventLoop::<PlatformEvent>::with_user_event().build()
            .map_err(|e| Error::Platform(e.to_string()))?;

        let proxy = event_loop.create_proxy();
        register_redraw_proxy(move || {
            let _ = proxy.send_event(PlatformEvent::RequestRedraw);
        });

        log::info!("Rupa: Starting Mobile Event Loop...");
        event_loop.run_app(self).map_err(|e| Error::Platform(e.to_string()))
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

        // Mobile usually doesn't have a visible mouse cursor to change
        core.cursor_pos = cursor_pos;
        core.requested_cursor = requested_cursor;
        core.pointer_capture = pointer_capture;
        core.focused_id = focused_id;
        
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

impl ApplicationHandler<PlatformEvent> for MobileRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // On Mobile, resumed is the primary entry point as the window 
        // can be destroyed and recreated when app goes to background.
        if self.window.is_none() {
            let attributes = winit::window::WindowAttributes::default();
            
            match event_loop.create_window(attributes) {
                Ok(window) => {
                    let core_lock = self.core.read().unwrap();
                    let _ = self.sync_metadata(&core_lock.metadata);
                    
                    // Dispatch initial safe area
                    let insets = crate::platform::mobile::infra::MobileInfra::get_safe_area_insets();
                    self.dispatch_event(InputEvent::SafeArea { 
                        top: insets.0, 
                        right: insets.1, 
                        bottom: insets.2, 
                        left: insets.3 
                    });

                    let size = window.inner_size();
                    let scale = window.scale_factor();
                    
                    let renderer = pollster::block_on(Renderer::new(
                        std::sync::Arc::new(window.clone()), 
                        size.width, 
                        size.height, 
                        scale as f32
                    ));
                    
                    self.window = Some(window);
                    self.renderer = Some(renderer);
                }
                Err(e) => log::error!("Failed to create Mobile window: {}", e),
            }
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        // Drop the renderer/surface on suspension to save resources/comply with OS rules.
        self.renderer = None;
        self.window = None;
        log::info!("Rupa: Suspended, resources released.");
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
            WindowEvent::Touch(touch) => {
                log::warn!("Mobile: Native touch mapping to Pointer events is currently unsupported.");
                log::trace!("Touch event: {:?}", touch);
            }
            _ => {}
        }
    }
}
