use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{WindowId, Window as WinitWindow};
use std::sync::Arc;

use rupa_core::{Component, Vec2, Error, Signal, Readable, CursorIcon, Renderer};
use rupa_core::events::InputEvent;
use crate::renderer::gui::renderer::Renderer as GuiRenderer;
use crate::platform::{SharedPlatformCore, runner::*, AppMetadata};
use crate::platform::dispatcher::InputDispatcher;

pub struct WebRunner {
    pub core: SharedPlatformCore,
    pub window: Option<Arc<WinitWindow>>,
    pub renderer: Option<GuiRenderer>,
    pub canvas_id: String,
}

impl WebRunner {
    pub fn new(core: SharedPlatformCore, canvas_id: impl Into<String>) -> Self {
        Self {
            core,
            window: None,
            renderer: None,
            canvas_id: canvas_id.into(),
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

        let (width, height) = (1024.0, 768.0);

        if let Some(root) = core.root.take() {
            // Compute layout
            let scene_node = core.scene.layout_engine.compute(
                root.as_ref(),
                renderer,
                width, 
                height
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
}

impl PlatformRunner for WebRunner {
    fn sync_metadata(&self, _metadata: &AppMetadata) -> Result<(), Error> {
        Ok(())
    }

    fn run(mut self) -> Result<(), Error> {
        let event_loop = EventLoop::<PlatformEvent>::with_user_event().build()
            .map_err(|e| Error::Platform(format!("Failed to build event loop: {}", e)))?;
            
        event_loop.run_app(&mut self)
            .map_err(|e| Error::Platform(format!("Failed to run app: {}", e)))
    }
}

impl ApplicationHandler<PlatformEvent> for WebRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let core_lock = self.core.read().unwrap();
            let title = core_lock.metadata.title.clone();
            
            let window_attributes = WinitWindow::default_attributes()
                .with_title(title);
            
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let size = window.inner_size();
            let renderer = pollster::block_on(GuiRenderer::new(window.clone(), size.width, size.height, 1.0));
            
            self.window = Some(window);
            self.renderer = Some(renderer);
        }
    }

    fn user_event(&mut self, _: &ActiveEventLoop, _event: PlatformEvent) {}

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                self.handle_redraw();
            }
            _ => {}
        }
    }
}
