use rupaui::prelude::*;

fn main() {
    // Initialize standard logging
    env_logger::init();
    
    // Bootstrap the Rupaui application with zero-boilerplate metadata
    App::new("Rupaui")
        .title("Rupaui | The Artisan's Playground")
        .version("1.0.0-alpha")
        .description("A semantic UI framework for the Rust ecosystem.")
        .author("Reasnov")
        .identifier("com.reasnov.rupaui")
        // Set the global body style (similar to CSS on <body>)
        .style(bg(Color::Semantic("background".into(), None)))
        .root(
            VStack::new()
                .w_full()
                .h_full()
                .items_center()
                .justify_center()
                .p(60.0)
                .gap(48.0)
                // --- Header with Theme Switcher ---
                .child(Box::new(
                    HStack::new()
                        .w(600.0)
                        .items_center()
                        .child(Box::new(
                            VStack::new()
                                .gap(12.0)
                                .child(Box::new(
                                    Text::new("RUPAUI")
                                        .style(text_color(Color::Semantic("primary".into(), None)))
                                ))
                                .child(Box::new(
                                    Text::new("The Artisan's Playground")
                                        .style(text_color(Color::Semantic("text".into(), None)))
                                ))
                        ))
                        .child(Box::new(
                            ThemeSwitcher::new()
                        ))
                ))
                // --- Deep Storytelling Philosophy ---
                .child(Box::new(
                    Section::new("Our Manifesto")
                        .style(w(600.0))
                        .child(Box::new(
                            VStack::new()
                                .gap(24.0)
                                .child(Box::new(
                                    Text::new("Beyond pixels and shaders, Rupaui is a testament to the harmony between logic and aesthetics.")
                                        .style(text_color(Color::Semantic("text-muted".into(), None)))
                                ))
                                .child(Box::new(
                                    Text::new("We believe that an interface should not just be seen, but understood. By strictly separating the 'Brain' from the 'Body', we ensure that every interaction is meaningful, every render is deliberate, and every state transition is predictable.")
                                        .style(text_color(Color::Semantic("text-muted".into(), None)))
                                ))
                                .child(Box::new(
                                    Text::new("Secure. Sustain. Scalable. This is the 3S Doctrine that drives our architectural soul.")
                                        .style(text_color(Color::Semantic("text".into(), None)))
                                ))
                        ))
                ))
                // --- Gratitude & Community ---
                .child(Box::new(
                    VStack::new()
                        .style(w(600.0))
                        .gap(16.0)
                        .child(Box::new(
                            Text::new("To the Builders")
                                .style(text_color(Color::Semantic("text".into(), None)))
                        ))
                        .child(Box::new(
                            Text::new("Thank you for choosing Rupaui. Your commitment to high-fidelity engineering inspires us to push the boundaries of what a Rust UI framework can achieve.")
                                .style(text_color(Color::Semantic("text-muted".into(), None)))
                        ))
                ))
                // --- Framework Credits ---
                .child(Box::new(
                    HStack::new()
                        .style(w(600.0))
                        .p(24.0)
                        .bg(Color::Semantic("surface".to_string(), None))
                        .style(rounded(16.0))
                        .child(Box::new(
                            Text::new("Handcrafted with architectural rigor by Reasnov.")
                                .style(text_color(Color::Semantic("text-muted".into(), Some(0.5))))
                        ))
                ))
        )
        .run();
}
