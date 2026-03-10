use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RupaApp {
    // Mesin reaktivitas yang dibagikan ke JS
}

#[wasm_bindgen]
impl RupaApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {}
    }

    /// Membuat "Signal" dari JS.
    /// JS bisa memanggil: const count = app.create_signal(0);
    pub fn create_signal(&self, value: JsValue) -> Result<JsValue, JsValue> {
        // Logika pembuatan Signal Rust yang bisa dibaca JS
        Ok(value)
    }

    /// Mengirim event ke komponen Rupa di WASM.
    pub fn dispatch_event(&self, event_json: String) {
        log::info!("Rupa Framework: Event received from JS: {}", event_json);
    }
}

// Helper untuk integrasi Web Components (Masa Depan)
pub mod custom_elements {
    // Jembatan untuk mendaftarkan Rupa sebagai <rupa-element />
}
