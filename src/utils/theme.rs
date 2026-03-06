use std::collections::HashMap;
use std::sync::{RwLock};
use once_cell::sync::Lazy;
use crate::utils::color::Color;
use crate::utils::style::Style;
use crate::utils::typography::FontWeight;
use crate::utils::border::{BorderStyle, Rounding};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ColorMode { #[default] Dark, Light, System }

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Variant { Primary, Secondary, Success, Warning, Danger, Info, Ghost, Link }

#[derive(Clone, Debug)]
pub struct Theme {
    pub mode: ColorMode,
    pub colors: HashMap<String, Color>,
    pub variants: HashMap<Variant, Color>,
    pub spacing: f32,
    pub borders: BorderConfig,
    pub typography: TypographyConfig,
    pub component_presets: HashMap<String, Style>,
}

#[derive(Clone, Debug)]
pub struct BorderConfig { pub width: f32, pub style: BorderStyle, pub radius: f32 }

#[derive(Clone, Debug)]
pub struct TypographyConfig { pub base_size: f32, pub family: String, pub weight: FontWeight }

static CURRENT_THEME: Lazy<RwLock<Theme>> = Lazy::new(|| RwLock::new(Theme::artisan_dark()));

impl Theme {
    pub fn current() -> Theme { CURRENT_THEME.read().unwrap().clone() }
    pub fn update<F>(f: F) where F: FnOnce(&mut Theme) { let mut theme = CURRENT_THEME.write().unwrap(); f(&mut theme); }

    pub fn apply_defaults(&self, style: &mut Style) {
        style.border.width = self.borders.width;
        style.border.style = self.borders.style.clone();
        style.rounding = Rounding::all(self.borders.radius);
        style.typography.size = Some(self.typography.base_size);
        style.typography.family = Some(self.typography.family.clone());
    }

    pub fn artisan_dark() -> Self {
        let mut colors = HashMap::new();
        let mut variants = HashMap::new();
        variants.insert(Variant::Primary, Color::indigo(600));
        variants.insert(Variant::Secondary, Color::slate(600));
        variants.insert(Variant::Success, Color::emerald(500));
        variants.insert(Variant::Danger, Color::red(500));
        colors.insert("background".into(), Color::slate(950));
        colors.insert("text".into(), Color::slate(50));
        colors.insert("surface".into(), Color::slate(900));

        Self {
            mode: ColorMode::Dark, colors, variants, spacing: 16.0,
            borders: BorderConfig { width: 1.0, style: BorderStyle::Solid, radius: 4.0 },
            typography: TypographyConfig { base_size: 16.0, family: "Inter".into(), weight: FontWeight::Regular },
            component_presets: HashMap::new(),
        }
    }

    pub fn variant(v: Variant) -> Color {
        let theme = CURRENT_THEME.read().unwrap();
        theme.variants.get(&v).cloned().unwrap_or(Color::Semantic("primary".into(), None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_theme_load() { let theme = Theme::current(); assert_eq!(theme.mode, ColorMode::Dark); }
    #[test]
    fn test_variant_resolution() {
        let color = Theme::variant(Variant::Primary);
        match color { Color::Semantic(name, _) => assert!(name.contains("indigo")), _ => {} }
    }
}
