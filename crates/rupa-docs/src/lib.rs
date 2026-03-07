use rupa::prelude::*;

/// Fungsi utama untuk membangun UI Dokumentasi Rupa.
/// Kode ini akan dijalankan di Desktop (GPU), Terminal (TUI), maupun Web (DOM).
pub fn docs_app() -> impl Component {
    VStack::new()
        .style((p(20.0), gap(10.0), bg_primary()))
        .child(Box::new(Text::new("Rupa Framework Documentation")
            .style(font_bold())))
        .child(Box::new(Text::new("The polyglot, full-stack UI framework for Rust artisans.")))
        .child(Box::new(Button::new("Get Started")
            .variant(Variant::Secondary)
            .on_click(|_| log::info!("Documentation: Get Started clicked!"))))
}
