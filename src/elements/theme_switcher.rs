use crate::utils::{Style, StyleModifier, generate_id, Theme, ColorMode, Variant};
use crate::Component;
use crate::elements::Button;

/// A semantic component that allows users to toggle between Light, Dark, and System theme modes.
pub struct ThemeSwitcher {
    pub id: String,
    pub style: Style,
}

impl ThemeSwitcher {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);

        Self {
            id: generate_id(),
            style,
        }
    }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }

    /// Cycles to the next theme mode: Dark -> Light -> System -> Dark
    pub fn cycle_mode() {
        Theme::update(|t| {
            t.mode = match t.mode {
                ColorMode::Dark => ColorMode::Light,
                ColorMode::Light => ColorMode::System,
                ColorMode::System => ColorMode::Dark,
            };
            log::info!("Theme mode changed to: {:?}", t.mode);
        });
    }
}

impl Component for ThemeSwitcher {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        let current_mode = Theme::current().mode;
        let label = match current_mode {
            ColorMode::Dark => "🌙 Dark",
            ColorMode::Light => "☀️ Light",
            ColorMode::System => "🖥️ System",
        };

        log::debug!("Rendering ThemeSwitcher [{}] (Current: {:?})", self.id, current_mode);
        
        // Internally it can render as a Button
        Button::new(label)
            .variant(Variant::Secondary)
            .style(self.style.clone())
            .on_click(|| Self::cycle_mode())
            .render();
    }
}
