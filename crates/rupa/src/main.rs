use rupa::prelude::*;

fn main() {
    // Initialize standard logging for artisan diagnostics
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    // Bootstrap the Rupa application with zero-boilerplate metadata
    App::new("Rupa")
        .title("Rupa | The Artisan Meta-Framework")
        .version("0.1.0-zenith")
        .description("A modular ecosystem for high-fidelity Rust applications.")
        .author("Reasnov & The Rupa Contributors")
        .identifier("io.rupa.showroom")
        // Set the global body style
        .style(bg(Color::Semantic("background".into(), None)))
        .root(
            VStack::new()
                .w_full()
                .h_full()
                .items_center()
                .justify_center()
                .p(80.0)
                .gap(64.0)
                // --- Brand Identity ---
                .child(Box::new(
                    VStack::new()
                        .items_center()
                        .gap(16.0)
                        .child(Box::new(
                            Text::new("RUPA")
                                .style(text_color(Color::Semantic("primary".into(), None)))
                                .style(font_size(48.0))
                                .style(font_bold())
                        ))
                        .child(Box::new(
                            Text::new("The Artisan's Workshop")
                                .style(text_color(Color::Semantic("text-muted".into(), None)))
                                .style(font_size(18.0))
                        ))
                ))
                // --- Philosophical Narrative ---
                .child(Box::new(
                    Container::new()
                        .style(w(700.0))
                        .child(Box::new(
                            VStack::new()
                                .gap(32.0)
                                .child(Box::new(
                                    Text::new("In the silence of the compiler, we find clarity.")
                                        .style(text_color(Color::Semantic("text".into(), None)))
                                        .style(font_size(24.0))
                                ))
                                .child(Box::new(
                                    Text::new("Rupa is not merely a library of widgets; it is a philosophy of construction. We reject the ephemeral magic of the monolithic, embracing instead the enduring truth of Atomic Materials and Composite Assemblies.")
                                        .style(text_color(Color::Semantic("text-muted".into(), None)))
                                ))
                                .child(Box::new(
                                    Text::new("Every Signal is a pulse. Every VNode is a cell. Every Patch is a healing touch to the screen. By strictly separating the 'Brain' from the 'Body', we ensure that every interaction is meaningful, every render is deliberate, and every state transition is predictable.")
                                        .style(text_color(Color::Semantic("text-muted".into(), None)))
                                ))
                                .child(Box::new(
                                    Text::new("Secure. Sustain. Scalable. This is the 3S Doctrine that drives our architectural soul.")
                                        .style(text_color(Color::Semantic("text".into(), None)))
                                        .style(font_bold())
                                ))
                        ))
                ))
                // --- Call to Action ---
                .child(Box::new(
                    HStack::new()
                        .gap(24.0)
                        .items_center()
                        .child(Box::new(
                            Button::new("Explore the Repository")
                                .variant(Variant::Primary)
                                .style(p(16.0))
                                .style(rounded(12.0))
                                .on_click(|_| {
                                    log::info!("Redirecting to: https://github.com/rupa-labs/rupa");
                                })
                        ))
                        .child(Box::new(
                            ThemeSwitcher::new()
                        ))
                ))
                // --- Footer ---
                .child(Box::new(
                    Text::new("Handcrafted with architectural rigor.")
                        .style(text_color(Color::Semantic("text-muted".into(), Some(0.3))))
                        .style(font_size(12.0))
                ))
        )
        .run();
}
