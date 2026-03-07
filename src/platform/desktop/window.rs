use winit::window::{Window as WinitWindow, WindowAttributes};
use winit::event_loop::ActiveEventLoop;
use winit::error::OsError;
use std::sync::Arc;

/// A platform-agnostic wrapper for the OS Window.
/// This prevents higher layers from being locked into Winit.
pub struct Window {
    inner: Arc<WinitWindow>,
}

impl Window {
    pub fn new(
        event_loop: &ActiveEventLoop,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Self, OsError> {
        let attributes = WindowAttributes::default()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height));

        let window = event_loop.create_window(attributes)?;

        Ok(Self {
            inner: Arc::new(window),
        })
    }

    /// Provides access to the underlying handle. 
    /// Required by Layer 2 (Rendering) for WGPU surface creation.
    pub fn handle(&self) -> Arc<WinitWindow> {
        self.inner.clone()
    }

    pub fn size(&self) -> (u32, u32) {
        let size = self.inner.inner_size();
        (size.width, size.height)
    }

    pub fn scale_factor(&self) -> f64 {
        self.inner.scale_factor()
    }

    pub fn request_redraw(&self) {
        self.inner.request_redraw();
    }
}
