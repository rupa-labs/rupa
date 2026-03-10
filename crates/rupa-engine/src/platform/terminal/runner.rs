use rupa_core::{Component, Vec2, Signal, generate_id, Error, CursorIcon, Renderer};
use rupa_core::events::InputEvent;
use crate::platform::dispatcher::InputDispatcher;

use crate::platform::{SharedPlatformCore, runner::*, register_redraw_proxy, AppMetadata};
use crate::renderer::tui::TuiRenderer;


pub struct TerminalRunner {
    pub core: SharedPlatformCore,
    pub renderer: TuiRenderer,
}

impl TerminalRunner {
    pub fn new(core: SharedPlatformCore) -> Self {
        Self {
            core,
            renderer: TuiRenderer::new(),
        }
    }

    fn handle_redraw(&mut self) {
        let mut core = match self.core.write() {
            Ok(c) => c,
            Err(_) => return,
        };

        if let Some(root) = core.root.take() {
            // Compute layout
            let scene_node = core.scene.layout_engine.compute(
                root.as_ref(),
                &self.renderer,
                self.renderer.width() as f32, 
                self.renderer.height() as f32
            );

            root.paint(
                &mut self.renderer,
                &core.scene.layout_engine.taffy,
                scene_node.raw(),
                false, 
                Vec2::zero(),
            );
            self.renderer.present();
            core.root = Some(root);
        }
    }

    fn _dispatch_event(&mut self, event: InputEvent) {
        let mut core = match self.core.write() {
            Ok(c) => c,
            Err(_) => return,
        };
        
        let mut cursor_pos = core.cursor_pos;
        let mut requested_cursor = core.requested_cursor;
        let mut pointer_capture = core.pointer_capture.take();
        let mut focused_id = core.focused_id.take();
        let event_listeners = core.event_listeners.clone();
        
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
                &event_listeners,
                core.debug,
            );
        }

        core.cursor_pos = cursor_pos;
        core.requested_cursor = requested_cursor;
        core.pointer_capture = pointer_capture;
        core.focused_id = focused_id;
    }
}

impl PlatformRunner for TerminalRunner {
    fn sync_metadata(&self, _metadata: &AppMetadata) -> Result<(), Error> {
        Ok(())
    }

    fn run(mut self) -> Result<(), Error> {
        log::info!("Terminal Runner started.");
        
        register_redraw_proxy(Box::new(|| {}));

        loop {
            self.handle_redraw();
            std::thread::sleep(std::time::Duration::from_millis(16));
            if false { break; }
        }
        Ok(())
    }
}
