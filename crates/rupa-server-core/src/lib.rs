use rupa_core::component::Component;

pub struct HtmlRenderer;

impl HtmlRenderer {
    /// Merender komponen Rupa menjadi string HTML.
    pub fn render_to_string(_root: &dyn Component) -> String {
        // Logika SSR: Mengunjungi tree komponen dan menghasilkan tag HTML
        format!("<div id=\"rupa-app\">{}</div>", "<!-- SSR Content -->")
    }
}

pub mod axum_handler {
    // Integrasi Axum untuk melayani request SSR
}
