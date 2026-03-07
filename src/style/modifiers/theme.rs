use std::collections::HashMap;
use crate::style::utilities::color::Color;
use crate::style::utilities::style::Style;
use crate::style::utilities::border::Rounding;
use crate::style::utilities::spacing::Spacing;
use crate::support::state::{Signal, Readable};
use once_cell::sync::Lazy;

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub enum Variant { #[default] Primary, Secondary, Success, Danger, Warning, Info, Light, Dark }

#[derive(Clone, Debug, PartialEq, Default, Eq)]
pub enum ColorMode { #[default] System, Light, Dark }

#[derive(Debug)]
pub struct Theme {
    pub mode: ColorMode,
    pub colors: HashMap<String, Color>,
    pub variants: HashMap<Variant, Color>,
    pub spacing: SpacingScale,
    pub borders: BorderScale,
}

#[derive(Debug, Clone)]
pub struct SpacingScale { pub base: f32, pub unit: f32 }
#[derive(Debug, Clone)]
pub struct BorderScale { pub radius: f32, pub width: f32 }

static THEME: Lazy<Signal<Theme>> = Lazy::new(|| {
    let mut variants = HashMap::new();
    variants.insert(Variant::Primary, Color::Semantic("primary".into(), None));
    Signal::new(Theme {
        mode: ColorMode::Dark,
        colors: HashMap::new(),
        variants,
        spacing: SpacingScale { base: 16.0, unit: 4.0 },
        borders: BorderScale { radius: 8.0, width: 1.0 },
    })
});

impl Theme {
    pub fn current() -> Theme { THEME.get() }
    pub fn signal() -> Signal<Theme> { THEME.clone() }
    
    pub fn update<F>(f: F) where F: FnOnce(&mut Theme) { 
        THEME.update(f);
    }

    pub fn set_mode(mode: ColorMode) {
        Self::update(|t| t.mode = mode);
    }

    pub fn toggle_mode() {
        let current = Self::current().mode;
        let next = match current {
            ColorMode::Dark => ColorMode::Light,
            _ => ColorMode::Dark,
        };
        Self::set_mode(next);
    }

    pub fn variant(v: Variant) -> Color { Self::current().variants.get(&v).cloned().unwrap_or(Color::Rgba(0.0, 0.0, 0.0, 1.0)) }
    
    pub fn apply_defaults(&self, style: &mut Style) {
        style.rounding = Rounding::all(self.borders.radius);
        style.padding = Spacing::all(self.spacing.base);
    }
}

impl Clone for Theme {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode.clone(),
            colors: self.colors.clone(),
            variants: self.variants.clone(),
            spacing: self.spacing.clone(),
            borders: self.borders.clone(),
        }
    }
}
