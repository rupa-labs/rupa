#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{HtmlCanvasElement, Window as WebWindow};

/// Wrapper for Browser-specific infrastructure.
/// Adheres to Dependency Inversion by isolating web-sys calls.
pub struct WebInfra;

impl WebInfra {
    #[cfg(target_arch = "wasm32")]
    pub fn get_canvas(canvas_id: &str) -> Result<HtmlCanvasElement, String> {
        let window = web_sys::window().ok_or("No global window found")?;
        let document = window.document().ok_or("No document found")?;
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or_else(|| format!("Canvas with id '{}' not found", canvas_id))?;
        
        canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Element is not a Canvas".into())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn fit_to_window(canvas: &HtmlCanvasElement) {
        let window = web_sys::window().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
        canvas.set_width(width);
        canvas.set_height(height);
    }
}
