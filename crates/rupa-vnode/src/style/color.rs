use crate::style::theme::Theme;
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
                let theme = Theme::current();
                let palette = theme.active_palette();
                let a = alpha.unwrap_or(1.0);
                
                let mut rgba = match name.as_str() {
                    "primary" => palette.primary.to_rgba(),
                    "background" => palette.background.to_rgba(),
                    "surface" => palette.surface.to_rgba(),
                    "text" => palette.text.to_rgba(),
                    "text-muted" | "text-dim" => palette.text_dim.to_rgba(),
                    "success" => palette.success.to_rgba(),
                    "warning" => palette.warning.to_rgba(),
                    "danger" | "error" => palette.danger.to_rgba(),
                    "border" => palette.border.to_rgba(),
                    _ => [1.0, 1.0, 1.0, 1.0],
                };
                rgba[3] *= a;
                rgba
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
