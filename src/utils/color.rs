/// Represents a color in various formats.
/// Normalized to RGBA [f32; 4] for GPU consumption.
#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Rgba(f32, f32, f32, f32),
    Hsla(f32, f32, f32, f32),
    Oklch(f32, f32, f32, f32),
    Hex(String),
    
    // --- Artisan Procedural Palette (50-950) ---
    Slate(u16),
    Gray(u16),
    Zinc(u16),
    Red(u16),
    Orange(u16),
    Amber(u16),
    Yellow(u16),
    Lime(u16),
    Green(u16),
    Emerald(u16),
    Teal(u16),
    Cyan(u16),
    Sky(u16),
    Blue(u16),
    Indigo(u16),
    Violet(u16),
    Purple(u16),
    Fuchsia(u16),
    Pink(u16),
    Rose(u16),

    Semantic(String, Option<u16>),

    /// A wrapper to apply alpha to any existing color (DRY).
    WithAlpha(Box<Color>, f32),
}

impl Color {
    /// Utility to apply alpha/opacity to any color variant.
    pub fn a(self, alpha: f32) -> Self {
        Color::WithAlpha(Box::new(self), alpha.clamp(0.0, 1.0))
    }

    /// Alias for .a()
    pub fn opacity(self, alpha: f32) -> Self {
        self.a(alpha)
    }

    pub fn normalize(&self) -> [f32; 4] {
        match self {
            Color::Rgba(r, g, b, a) => [*r, *g, *b, *a],
            Color::Hsla(h, s, l, a) => {
                let [r, g, b] = hsla_to_rgba(*h, *s, *l);
                [r, g, b, *a]
            }
            Color::Oklch(l, c, h, a) => {
                let [r, g, b] = oklch_to_rgba(*l, *c, *h);
                [r, g, b, *a]
            }
            Color::Hex(hex) => hex_to_rgba(hex),
            
            Color::Slate(s) => generate_artisan_color(210.0, 0.02, *s),
            Color::Gray(s) => generate_artisan_color(0.0, 0.0, *s),
            Color::Zinc(s) => generate_artisan_color(240.0, 0.01, *s),
            Color::Red(s) => generate_artisan_color(29.0, 0.23, *s),
            Color::Orange(s) => generate_artisan_color(55.0, 0.18, *s),
            Color::Amber(s) => generate_artisan_color(75.0, 0.15, *s),
            Color::Yellow(s) => generate_artisan_color(95.0, 0.14, *s),
            Color::Lime(s) => generate_artisan_color(125.0, 0.19, *s),
            Color::Green(s) => generate_artisan_color(145.0, 0.17, *s),
            Color::Emerald(s) => generate_artisan_color(165.0, 0.16, *s),
            Color::Teal(s) => generate_artisan_color(185.0, 0.12, *s),
            Color::Cyan(s) => generate_artisan_color(215.0, 0.11, *s),
            Color::Sky(s) => generate_artisan_color(245.0, 0.13, *s),
            Color::Blue(s) => generate_artisan_color(265.0, 0.18, *s),
            Color::Indigo(s) => generate_artisan_color(285.0, 0.17, *s),
            Color::Violet(s) => generate_artisan_color(305.0, 0.20, *s),
            Color::Purple(s) => generate_artisan_color(325.0, 0.19, *s),
            Color::Fuchsia(s) => generate_artisan_color(345.0, 0.22, *s),
            Color::Pink(s) => generate_artisan_color(5.0, 0.18, *s),
            Color::Rose(s) => generate_artisan_color(15.0, 0.19, *s),

            Color::Semantic(name, shade) => {
                let s = shade.unwrap_or(500);
                match name.as_str() {
                    "red" => Color::Red(s).normalize(),
                    "blue" => Color::Blue(s).normalize(),
                    "green" => Color::Green(s).normalize(),
                    _ => semantic_to_rgba(name),
                }
            }

            // The alpha override logic
            Color::WithAlpha(inner, alpha) => {
                let [r, g, b, _] = inner.normalize();
                [r, g, b, *alpha]
            }
        }
    }
}

/// The Mathematical Engine of Artisan Colors
fn generate_artisan_color(hue: f32, max_chroma: f32, shade: u16) -> [f32; 4] {
    let shade = shade.clamp(50, 950) as f32;
    let l = 1.0 - (shade - 50.0) / 1000.0 * 0.9;
    let chroma_boost = if shade < 500.0 { (shade / 500.0).powf(0.5) } else { ((1000.0 - shade) / 500.0).powf(0.5) };
    let c = max_chroma * chroma_boost;
    let [r, g, b] = oklch_to_rgba(l, c, hue);
    [r, g, b, 1.0]
}

fn oklch_to_rgba(l: f32, c: f32, h: f32) -> [f32; 3] {
    let h_rad = h.to_radians();
    let a = c * h_rad.cos();
    let b = c * h_rad.sin();
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;
    let l = l_ * l_ * l_; let m = m_ * m_ * m_; let s = s_ * s_ * s_;
    let r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
    let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
    let b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;
    [r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0)]
}

fn hsla_to_rgba(h: f32, s: f32, l: f32) -> [f32; 3] {
    let h = h % 360.0;
    let s = s.clamp(0.0, 1.0);
    let l = l.clamp(0.0, 1.0);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = if h < 60.0 { (c, x, 0.0) } else if h < 120.0 { (x, c, 0.0) } else if h < 180.0 { (0.0, c, x) } else if h < 240.0 { (0.0, x, c) } else if h < 300.0 { (x, 0.0, c) } else { (c, 0.0, x) };
    [r + m, g + m, b + m]
}

fn hex_to_rgba(hex: &str) -> [f32; 4] {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
        [r, g, b, 1.0]
    } else if hex.len() == 8 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
        let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255) as f32 / 255.0;
        [r, g, b, a]
    } else { [0.0, 0.0, 0.0, 1.0] }
}

fn semantic_to_rgba(name: &str) -> [f32; 4] {
    match name.to_lowercase().as_str() {
        "white" => [1.0, 1.0, 1.0, 1.0],
        "black" => [0.0, 0.0, 0.0, 1.0],
        "transparent" => [0.0, 0.0, 0.0, 0.0],
        _ => [0.0, 0.0, 0.0, 1.0],
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        if s.starts_with('#') { Color::Hex(s.to_string()) } else { Color::Semantic(s.to_string(), None) }
    }
}

impl From<[f32; 4]> for Color {
    fn from(rgba: [f32; 4]) -> Self { Color::Rgba(rgba[0], rgba[1], rgba[2], rgba[3]) }
}
