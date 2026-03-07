use rupaui::prelude::*;

fn main() {
    // Initialize standard logging for WGPU and Platform feedback
    env_logger::init();
    
    // Bootstrap the Rupaui application with a storytelling Hero Screen
    App::new("Rupaui | The Soul of UI")
        .root(
            VStack::new()
                .bg(Color::Semantic("background".to_string(), None))
                .p(60.0)
                .gap(48.0)
                // --- Header with Theme Switcher ---
                .child(Box::new(
                    HStack::new()
                        .child(Box::new(
                            VStack::new()
                                .gap(12.0)
                                .child(Box::new(
                                    Text::new("RUPAUI")
                                        .style(text_color(Color::Semantic("primary".into(), None)))
                                ))
                                .child(Box::new(
                                    Text::new("The Soul of Semantic Interfaces")
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
