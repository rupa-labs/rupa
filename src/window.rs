use winit::window::{Window as WinitWindow, WindowAttributes};
use winit::event_loop::ActiveEventLoop;
use winit::error::OsError;
use std::sync::Arc;

/// The primary viewport for Rupaui applications.
/// Optimized for both native desktop and WebAssembly environments.
pub struct Window {
    raw: Arc<WinitWindow>,
    title: String,
}

impl Window {
    /// Creates a new window. On Web, it can attach to a canvas via ID.
    pub fn new(
        event_loop: &ActiveEventLoop,
        title: &str,
        width: u32,
        height: u32,
        #[cfg(target_arch = "wasm32")] canvas_id: Option<&str>,
    ) -> Result<Self, OsError> {
        let mut attributes = WindowAttributes::default()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height));

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowAttributesExtWeb;
            if let Some(id) = canvas_id {
                let document = web_sys::window().unwrap().document().unwrap();
                let canvas = document.get_element_by_id(id).unwrap();
                attributes = attributes.with_canvas(Some(canvas.unchecked_into()));
            } else {
                attributes = attributes.with_append(true);
            }
        }

        let raw = Arc::new(event_loop.create_window(attributes)?);

        Ok(Self {
            raw,
            title: title.to_string(),
        })
    }

    pub fn id(&self) -> winit::window::WindowId {
        self.raw.id()
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
