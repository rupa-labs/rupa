#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{HtmlCanvasElement, Window as WebWindow};

use crate::platform::app::IconSource;

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

    #[cfg(target_arch = "wasm32")]
    pub fn set_favicon(source: &IconSource) -> Result<(), String> {
        let window = web_sys::window().ok_or("No global window found")?;
        let document = window.document().ok_or("No document found")?;
        let head = document.head().ok_or("No <head> found")?;

        let link = document
            .query_selector("link[rel*='icon']")
            .map_err(|_| "Query selector failed")?
            .unwrap_or_else(|| {
                let el = document.create_element("link").unwrap();
                el.set_attribute("rel", "icon").unwrap();
                head.append_child(&el).unwrap();
                el
            });

        let url = match source {
            IconSource::Path(path) => path.clone(),
            IconSource::Bytes(bytes) => {
                let b64 = base64::encode(bytes);
                format!("data:image/png;base64,{}", b64)
            }
        };

        link.set_attribute("href", &url).map_err(|_| "Failed to set href".into())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn set_meta_theme_color(rgba: [f32; 4]) -> Result<(), String> {
        let window = web_sys::window().ok_or("No global window found")?;
        let document = window.document().ok_or("No document found")?;
        let head = document.head().ok_or("No <head> found")?;

        let meta = document
            .query_selector("meta[name='theme-color']")
            .map_err(|_| "Query selector failed")?
            .unwrap_or_else(|| {
                let el = document.create_element("meta").unwrap();
                el.set_attribute("name", "theme-color").unwrap();
                head.append_child(&el).unwrap();
                el
            });

        let hex = format!("#{:02x}{:02x}{:02x}", 
            (rgba[0] * 255.0) as u8, 
            (rgba[1] * 255.0) as u8, 
            (rgba[2] * 255.0) as u8
        );
        meta.set_attribute("content", &hex).map_err(|_| "Failed to set content".into())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn push_state(path: &str) -> Result<(), String> {
        let window = web_sys::window().ok_or("No global window found")?;
        let history = window.history().map_err(|_| "Failed to get history")?;
        history
            .push_state_with_url(&JsValue::NULL, "", Some(path))
            .map_err(|_| "Failed to push state".into())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_current_path() -> String {
        web_sys::window()
            .and_then(|w| w.location().pathname().ok())
            .unwrap_or_else(|| "/".to_string())
    }
}
