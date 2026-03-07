use winit::window::{Window as WinitWindow, WindowAttributes};
use winit::event_loop::ActiveEventLoop;
use std::sync::Arc;

/// Wrapper for Desktop-specific infrastructure (Winit).
/// Adheres to Dependency Inversion by isolating Winit window creation.
pub struct DesktopInfra;

impl DesktopInfra {
    pub fn create_window(
        event_loop: &ActiveEventLoop,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Arc<WinitWindow>, Box<dyn std::error::Error>> {
        let attributes = WindowAttributes::default()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height));

        let window = event_loop.create_window(attributes)?;
        Ok(Arc::new(window))
    }
}
