use std::collections::HashMap;
use crate::color::Color;
use crate::types::{Unit, FontWeight};
use crate::style::typography::FontFamily;
use rupa_signals::Signal;
use once_cell::sync::Lazy;

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Variant { #[default] Primary, Secondary, Success, Danger, Warning, Info, Light, Dark }

#[derive(Clone, Debug, PartialEq, Default, Eq, serde::Serialize, serde::Deserialize)]
pub enum ColorMode { #[default] System, Light, Dark }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub text_dim: Color,
    pub primary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub border: Color,
}

impl Palette {
    pub fn light() -> Self {
        Self {
            background: Color::Rgba(0.98, 0.98, 0.98, 1.0),
            surface: Color::Rgba(1.0, 1.0, 1.0, 1.0),
            text: Color::Rgba(0.1, 0.1, 0.1, 1.0),
            text_dim: Color::Rgba(0.4, 0.4, 0.4, 1.0),
            primary: Color::Rgba(0.0, 0.47, 1.0, 1.0),
            success: Color::Rgba(0.13, 0.75, 0.42, 1.0),
            warning: Color::Rgba(1.0, 0.75, 0.0, 1.0),
            danger: Color::Rgba(1.0, 0.22, 0.14, 1.0),
            border: Color::Rgba(0.88, 0.88, 0.88, 1.0),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: Color::Rgba(0.05, 0.05, 0.05, 1.0),
            surface: Color::Rgba(0.12, 0.12, 0.12, 1.0),
            text: Color::Rgba(0.95, 0.95, 0.95, 1.0),
            text_dim: Color::Rgba(0.6, 0.6, 0.6, 1.0),
            primary: Color::Rgba(0.2, 0.6, 1.0, 1.0),
            success: Color::Rgba(0.2, 0.8, 0.4, 1.0),
            warning: Color::Rgba(1.0, 0.8, 0.2, 1.0),
            danger: Color::Rgba(1.0, 0.3, 0.3, 1.0),
            border: Color::Rgba(0.25, 0.25, 0.25, 1.0),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct SpacingScale { pub base: Unit, pub unit: Unit }
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct BorderScale { pub radius: Unit, pub width: Unit }
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct TypographyScale { 
    pub base_size: Unit, 
    pub base_weight: FontWeight,
    pub base_family: FontFamily,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Theme {
    pub mode: ColorMode,
    pub light: Palette,
    pub dark: Palette,
    pub colors: HashMap<String, Color>,
    pub variants: HashMap<Variant, Color>,
    pub spacing: SpacingScale,
    pub borders: BorderScale,
    pub typography: TypographyScale,
}

static THEME: Lazy<Signal<Theme>> = Lazy::new(|| {
    let mut variants = HashMap::new();
    variants.insert(Variant::Primary, Color::Semantic("primary".into(), None));
    Signal::new(Theme {
        mode: ColorMode::Dark,
        light: Palette::light(),
        dark: Palette::dark(),
        colors: HashMap::new(),
        variants,
        spacing: SpacingScale { base: Unit::Absolute(16.0), unit: Unit::Absolute(4.0) },
        borders: BorderScale { radius: Unit::Absolute(8.0), width: Unit::Absolute(1.0) },
        typography: TypographyScale { 
            base_size: Unit::Absolute(16.0), 
            base_weight: FontWeight::Normal,
            base_family: FontFamily::Sans,
        },
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
    
    pub fn active_palette(&self) -> &Palette {
        match self.mode {
            ColorMode::Dark => &self.dark,
            _ => &self.light,
        }
    }

    pub fn apply_defaults(&self, style: &mut crate::style::style::Style) {
        let palette = self.active_palette();
        style.rounding = crate::border::Rounding::all(self.borders.radius.clone());
        style.padding = crate::spacing::Spacing::all(self.spacing.base.clone());
        style.background.color = Some(palette.background.clone());
        style.typography.color = Some(palette.text.clone());
        style.typography.size = self.typography.base_size.clone();
        style.typography.weight = self.typography.base_weight;
        style.typography.family = self.typography.base_family;
    }
}

impl Clone for Theme {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode.clone(),
            light: self.light.clone(),
            dark: self.dark.clone(),
            colors: self.colors.clone(),
            variants: self.variants.clone(),
            spacing: self.spacing.clone(),
            borders: self.borders.clone(),
            typography: self.typography.clone(),
        }
    }
}
