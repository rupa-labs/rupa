use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use crate::style::utilities::color::Color;
use crate::style::utilities::style::Style;
use crate::style::utilities::border::Rounding;
use crate::style::utilities::spacing::Spacing;

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub enum Variant { #[default] Primary, Secondary, Success, Danger, Warning, Info, Light, Dark }

#[derive(Clone, Debug, PartialEq, Default, Eq)]
pub enum ColorMode { #[default] System, Light, Dark }

pub struct Theme {
    pub mode: ColorMode,
    pub colors: HashMap<String, Color>,
    pub variants: HashMap<Variant, Color>,
    pub spacing: SpacingScale,
    pub borders: BorderScale,
}

pub struct SpacingScale { pub base: f32, pub unit: f32 }
pub struct BorderScale { pub radius: f32, pub width: f32 }

static THEME: Lazy<Arc<RwLock<Theme>>> = Lazy::new(|| {
    let mut variants = HashMap::new();
    variants.insert(Variant::Primary, Color::Semantic("indigo-600".into(), None));
    Arc::new(RwLock::new(Theme {
        mode: ColorMode::Dark,
        colors: HashMap::new(),
        variants,
        spacing: SpacingScale { base: 16.0, unit: 4.0 },
        borders: BorderScale { radius: 8.0, width: 1.0 },
    }))
});

impl Theme {
    pub fn current() -> Theme { THEME.read().unwrap().clone() }
    pub fn update<F>(f: F) where F: FnOnce(&mut Theme) { f(&mut *THEME.write().unwrap()); }
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
            spacing: SpacingScale { base: self.spacing.base, unit: self.spacing.unit },
            borders: BorderScale { radius: self.borders.radius, width: self.borders.width },
        }
    }
}
