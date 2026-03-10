use rupa_core::{Vec2, Error, CursorIcon, Renderer};
use rupa_engine::platform::{context::SharedPlatformCore, runner::PlatformRunner, app::AppMetadata};
use crate::renderer::renderer::WgpuRenderer;
use std::sync::Arc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{WindowBuilder, Window};

pub struct DesktopRunner {
    pub core: SharedPlatformCore,
    pub window: Option<Arc<Window>>,
}

impl DesktopRunner {
    pub fn new(core: SharedPlatformCore) -> Self {
        Self {
            core,
            window: None,
        }
    }

    fn handle_redraw(&mut self) {
        // GPU Redraw logic using WgpuRenderer...
    }
}

impl PlatformRunner for DesktopRunner {
    fn sync_metadata(&self, _metadata: &AppMetadata) -> Result<(), Error> {
        Ok(())
    }

    fn run(mut self) -> Result<(), Error> {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        self.window = Some(Arc::new(window));

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                Event::RedrawRequested(_) => {
                    if rupa_motion::GLOBAL_TIMELINE.tick() {
                        self.window.as_ref().unwrap().request_redraw();
                    }
                    self.handle_redraw();
                }
                _ => {}
            }
        }).unwrap();
        
        Ok(())
    }
}
