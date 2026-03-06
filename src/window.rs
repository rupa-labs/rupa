use winit::window::{Window as WinitWindow, WindowAttributes};
use winit::event_loop::ActiveEventLoop;
use winit::error::OsError;
use std::sync::Arc;

/// The primary viewport for Rupaui applications.
pub struct Window {
    raw: Arc<WinitWindow>,
}

impl Window {
    /// Creates a new native window instance.
    pub fn new(
        event_loop: &ActiveEventLoop,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Self, OsError> {
        log::debug!("Forging new OS Window: {} ({}x{})", title, width, height);

        let attributes = WindowAttributes::default()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height));

        let window = event_loop.create_window(attributes)?;

        Ok(Self {
            raw: Arc::new(window),
        })
    }

    pub fn raw(&self) -> Arc<WinitWindow> {
        self.raw.clone()
    }

    pub fn size(&self) -> (u32, u32) {
        let size = self.raw.inner_size();
        (size.width, size.height)
    }

    pub fn request_redraw(&self) {
        self.raw.request_redraw();
    }
}
