use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use crate::utils::color::Color;
use crate::utils::style::Style;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ColorMode {
    #[default]
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Variant {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Link,
}

/// Design Tokens for Rupaui.
#[derive(Debug, Clone, Default)]
pub struct Theme {
    pub mode: ColorMode,
    pub colors: HashMap<String, Color>,
    pub variants: HashMap<Variant, Color>,
    pub spacing: HashMap<String, f32>,
    pub styles: HashMap<String, Style>,
}

static CURRENT_THEME: Lazy<Arc<RwLock<Theme>>> = Lazy::new(|| {
    Arc::new(RwLock::new(Theme::default_artisan()))
});

impl Theme {
    pub fn current() -> Theme {
        CURRENT_THEME.read().unwrap().clone()
    }

    pub fn update<F>(f: F) where F: FnOnce(&mut Theme) {
        let mut theme = CURRENT_THEME.write().unwrap();
        f(&mut theme);
    }

    pub fn default_artisan() -> Self {
        let mut colors = HashMap::new();
        let mut variants = HashMap::new();
        
        // Semantic Variants mapping to Artisan Palette
        variants.insert(Variant::Primary, Color::Indigo(600));
        variants.insert(Variant::Secondary, Color::Slate(600));
        variants.insert(Variant::Success, Color::Emerald(500));
        variants.insert(Variant::Danger, Color::Red(500));
        variants.insert(Variant::Warning, Color::Amber(400));
        variants.insert(Variant::Info, Color::Cyan(400));
        variants.insert(Variant::Light, Color::Slate(100));
        variants.insert(Variant::Dark, Color::Slate(900));

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
            mode: ColorMode::Dark,
            colors,
            variants,
            spacing,
            styles: HashMap::new(),
        }
    }

    pub fn variant(v: Variant) -> Color {
        let theme = CURRENT_THEME.read().unwrap();
        theme.variants.get(&v).cloned().unwrap_or(Color::Slate(500))
    }

    pub fn color(name: &str) -> Color {
        let theme = CURRENT_THEME.read().unwrap();
        theme.colors.get(name).cloned().unwrap_or(Color::Semantic(name.to_string(), None))
    }
}
