use serde::{Serialize, Deserialize};

/// A high-precision RGBA color for the Rupa Framework.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Self = Self::rgba(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgba(0.0, 0.0, 0.0, 1.0);
    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);
    pub const RED: Self = Self::rgba(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::rgba(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::rgba(0.0, 0.0, 1.0, 1.0);

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Creates a color from OKLCH values (Lightness, Chroma, Hue in degrees, Alpha).
    /// OKLCH is the preferred standard for perceptual uniformity in Rupa.
    pub fn oklch(l: f32, c: f32, h: f32, a: f32) -> Self {
        let (r, g, b) = Self::oklch_to_rgba(l, c, h);
        Self::rgba(r, g, b, a)
    }

    /// Conversion from OKLCH to linear RGBA.
    fn oklch_to_rgba(l: f32, c: f32, h_deg: f32) -> (f32, f32, f32) {
        let h = h_deg.to_radians();
        let a = c * h.cos();
        let b = c * h.sin();
        let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
        let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
        let s_ = l - 0.0894841775 * a - 1.2914855480 * b;
        let l_r = l_ * l_ * l_;
        let m_r = m_ * m_ * m_;
        let s_r = s_ * s_ * s_;
        let r = 4.0767416621 * l_r - 3.3077115913 * m_r + 0.2309699292 * s_r;
        let g = -1.2684380046 * l_r + 2.6097574011 * m_r - 0.3413193965 * s_r;
        let b_ = -0.0041960863 * l_r - 0.7034186147 * m_r + 1.7076147010 * s_r;
        (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b_.clamp(0.0, 1.0))
    }

    /// Creates a color from hex-like integers (e.g., 0xFF0000 for Red).
    pub fn hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Self::rgb(r, g, b)
    }

    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    /// Linear interpolation between two colors.
    pub fn lerp(&self, other: Color, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
}

impl From<[f32; 4]> for Color {
    fn from(c: [f32; 4]) -> Self {
        Self::rgba(c[0], c[1], c[2], c[3])
    }
}

impl From<[f32; 3]> for Color {
    fn from(c: [f32; 3]) -> Self {
        Self::rgb(c[0], c[1], c[2])
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_hex() {
        let red = Color::hex(0xFF0000);
        assert_eq!(red.r, 1.0);
        assert_eq!(red.g, 0.0);
        assert_eq!(red.b, 0.0);
    }

    #[test]
    fn test_oklch_white() {
        let white = Color::oklch(1.0, 0.0, 0.0, 1.0);
        assert!(white.r > 0.99 && white.g > 0.99 && white.b > 0.99);
    }
}
