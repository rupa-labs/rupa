use crate::style::theme::{Theme, ColorMode};
use rupa_base::Color as RgbaColor;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Color {
    Rgba(f32, f32, f32, f32),
    Oklch(f32, f32, f32, f32), // Lightness, Chroma, Hue, Alpha
    Semantic(String, Option<f32>),
    Transparent,
}

impl Color {
    pub fn to_rgba(&self) -> [f32; 4] {
        match self {
            Color::Rgba(r, g, b, a) => [*r, *g, *b, *a],
            Color::Oklch(l, c, h, a) => {
                let rgba = RgbaColor::oklch(*l, *c, *h, *a);
                [rgba.r, rgba.g, rgba.b, rgba.a]
            },
            Color::Semantic(name, alpha) => {
                let mode = Theme::current().mode;
                let a = alpha.unwrap_or(1.0);
                match name.as_str() {
                    "primary" => match mode {
                        ColorMode::Light => [0.3, 0.4, 0.9, a],
                        _ => [0.39, 0.45, 1.0, a],
                    },
                    "background" => match mode {
                        ColorMode::Light => [0.98, 0.98, 0.98, a],
                        _ => [0.05, 0.05, 0.05, a],
                    },
                    "surface" => match mode {
                        ColorMode::Light => [1.0, 1.0, 1.0, a],
                        _ => [0.12, 0.12, 0.12, a],
                    },
                    "text" => match mode {
                        ColorMode::Light => [0.1, 0.1, 0.1, a],
                        _ => [0.95, 0.95, 0.95, a],
                    },
                    "text-muted" => match mode {
                        ColorMode::Light => [0.4, 0.4, 0.4, a],
                        _ => [0.6, 0.6, 0.6, a],
                    },
                    _ => [1.0, 1.0, 1.0, a],
                }
            },
            Color::Transparent => [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn white(a: f32) -> Self { Color::Rgba(1.0, 1.0, 1.0, a) }
    pub fn black(a: f32) -> Self { Color::Rgba(0.0, 0.0, 0.0, a) }
    
    // Helpers for common colors
    pub fn indigo(weight: u32) -> Self { Color::Semantic(format!("indigo-{}", weight), None) }
    pub fn slate(weight: u32) -> Self { Color::Semantic(format!("slate-{}", weight), None) }
    pub fn emerald(weight: u32) -> Self { Color::Semantic(format!("emerald-{}", weight), None) }
    pub fn red(weight: u32) -> Self { Color::Semantic(format!("red-{}", weight), None) }

    pub fn to_hex(&self) -> String {
        match self {
            Color::Rgba(r, g, b, _) => {
                format!("#{:02x}{:02x}{:02x}", 
                    (r * 255.0) as u8, 
                    (g * 255.0) as u8, 
                    (b * 255.0) as u8
                )
            },
            Color::Oklch(l, c, h, a) => {
                let rgba = RgbaColor::oklch(*l, *c, *h, *a);
                format!("#{:02x}{:02x}{:02x}", 
                    (rgba.r * 255.0) as u8, 
                    (rgba.g * 255.0) as u8, 
                    (rgba.b * 255.0) as u8
                )
            },
            Color::Semantic(name, _) => format!("var(--rupa-{})", name),
            Color::Transparent => "transparent".to_string(),
        }
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self { Color::Semantic(s.to_string(), None) }
}

impl From<RgbaColor> for Color {
    fn from(c: RgbaColor) -> Self {
        Color::Rgba(c.r, c.g, c.b, c.a)
    }
}
