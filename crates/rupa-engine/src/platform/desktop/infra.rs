use winit::window::{Window as WinitWindow, WindowAttributes, Icon};
use winit::event_loop::ActiveEventLoop;
use std::sync::Arc;
use crate::platform::app::IconSource;

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

    pub fn set_icon(window: &WinitWindow, source: &IconSource) -> Result<(), Box<dyn std::error::Error>> {
        let icon_data = match source {
            IconSource::Path(path) => {
                let img = image::open(path)?;
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                (rgba.into_raw(), width, height)
            }
            IconSource::Bytes(bytes) => {
                let img = image::load_from_memory(bytes)?;
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                (rgba.into_raw(), width, height)
            }
        };

        let icon = Icon::from_rgba(icon_data.0, icon_data.1, icon_data.2)?;
        window.set_window_icon(Some(icon));
        Ok(())
    }
}
