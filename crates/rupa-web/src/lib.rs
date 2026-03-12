//! # Rupa Web 🌐
//!
//! The Web Showroom for the Rupa Framework. This crate provides the 
//! **Adapters & Infrastructure** (Tier 3) to render Rupa applications 
//! in the browser via WebAssembly (WASM) and the DOM.

use wasm_bindgen::prelude::*;

/// The WASM entry point for a Rupa Web Application.
#[wasm_bindgen]
pub struct RupaApp {
    // The reactive engine state shared with JS
}

#[wasm_bindgen]
impl RupaApp {
    /// Initializes the Rupa Web Application and sets up panic hooks.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {}
    }

    /// Creates a Rupa Signal from JavaScript.
    /// JS can call: `const count = app.create_signal(0);`
    pub fn create_signal(&self, value: JsValue) -> Result<JsValue, JsValue> {
        // Rust Signal creation logic readable by JS
        Ok(value)
    }

    /// Dispatches an event to the Rupa components in WASM.
    pub fn dispatch_event(&self, event_json: String) {
        log::info!("Rupa Framework: Event received from JS: {}", event_json);
    }
}

/// Helpers for Web Components integration (Future Feature).
pub mod custom_elements {
    // Bridge for registering Rupa as a <rupa-element />
}
