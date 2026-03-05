use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use crate::utils::color::Color;
use crate::utils::style::Style;

/// Design Tokens for Rupaui.
/// Acts like CSS Variables for colors, spacing, and component presets.
#[derive(Debug, Clone, Default)]
pub struct Theme {
    pub colors: HashMap<String, Color>,
    pub spacing: HashMap<String, f32>,
    pub styles: HashMap<String, Style>,
    pub custom: HashMap<String, String>,
}

static CURRENT_THEME: Lazy<Arc<RwLock<Theme>>> = Lazy::new(|| {
    Arc::new(RwLock::new(Theme::default_artisan()))
});

impl Theme {
    /// Access the global active theme.
    pub fn current() -> Theme {
        CURRENT_THEME.read().unwrap().clone()
    }

    /// Update the global theme.
    pub fn update<F>(f: F) 
    where F: FnOnce(&mut Theme) {
        let mut theme = CURRENT_THEME.write().unwrap();
        f(&mut theme);
    }

    /// Default theme based on the Rupa Artisan aesthetic.
    pub fn default_artisan() -> Self {
        let mut colors = HashMap::new();
        colors.insert("primary".to_string(), Color::Indigo(600));
        colors.insert("secondary".to_string(), Color::Slate(600));
        colors.insert("accent".to_string(), Color::Amber(400));
        colors.insert("background".to_string(), Color::Slate(950));
        colors.insert("surface".to_string(), Color::Slate(900));
        colors.insert("text".to_string(), Color::Slate(50));

        let mut spacing = HashMap::new();
        spacing.insert("xs".to_string(), 4.0);
        spacing.insert("sm".to_string(), 8.0);
        spacing.insert("md".to_string(), 16.0);
        spacing.insert("lg".to_string(), 24.0);
        spacing.insert("xl".to_string(), 32.0);

        Self {
            colors,
            spacing,
            styles: HashMap::new(),
            custom: HashMap::new(),
        }
    }

    // --- Accessors ---

    pub fn color(name: &str) -> Color {
        let theme = CURRENT_THEME.read().unwrap();
        theme.colors.get(name).cloned().unwrap_or(Color::Semantic(name.to_string(), None))
    }

    pub fn space(name: &str) -> f32 {
        let theme = CURRENT_THEME.read().unwrap();
        *theme.spacing.get(name).unwrap_or(&0.0)
    }

    pub fn style(name: &str) -> Style {
        let theme = CURRENT_THEME.read().unwrap();
        theme.styles.get(name).cloned().unwrap_or_default()
    }
}
