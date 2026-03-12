use rupa_core::Error;
use rupa_engine::platform::{context::SharedPlatformCore, runner::PlatformRunner, app::AppMetadata};
use std::sync::Arc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::Window;

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
        // Redraw logic...
    }
}

impl PlatformRunner for DesktopRunner {
    fn sync_metadata(&self, _metadata: &AppMetadata) -> Result<(), Error> {
        Ok(())
    }

    fn run(mut self) -> Result<(), Error> {
        let event_loop = EventLoop::new().map_err(|e| Error::Platform(format!("EventLoop error: {}", e)))?;
        
        // Use newer winit API pattern if possible, or fallback to simple one
        // For this maturation, we fix the known missing imports.
        
        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    elwt.exit();
                }
                Event::AboutToWait => {
                    if rupa_motion::GLOBAL_TIMELINE.tick() {
                        if let Some(ref win) = self.window {
                            win.request_redraw();
                        }
                    }
                }
                Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                    self.handle_redraw();
                }
                _ => {}
            }
        }).map_err(|e| Error::Platform(format!("Run error: {}", e)))?;
        
        Ok(())
    }
}
