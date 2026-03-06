use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign};

/// A 2D Vector for high-precision SVG and Layout calculations.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn zero() -> Self { Self::new(0.0, 0.0) }
    pub fn one() -> Self { Self::new(1.0, 1.0) }

    pub fn length(&self) -> f32 { (self.x * self.x + self.y * self.y).sqrt() }
    
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 { *self / len } else { *self }
    }

    pub fn dot(&self, other: Vec2) -> f32 { self.x * other.x + self.y * other.y }
    
    pub fn distance(&self, other: Vec2) -> f32 { (*self - other).length() }

    pub fn lerp(&self, other: Vec2, t: f32) -> Self {
        *self + (other - *self) * t
    }

    pub fn rotate(&self, degrees: f32) -> Self {
        let rad = degrees.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();
        Self::new(
            self.x * cos - self.y * sin,
            self.x * sin + self.y * cos
        )
    }

    pub fn angle(&self) -> f32 { self.y.atan2(self.x).to_degrees() }
}

// --- Operator Overloads ---

impl Add for Vec2 { type Output = Self; fn add(self, other: Self) -> Self { Self::new(self.x + other.x, self.y + other.y) } }
impl Sub for Vec2 { type Output = Self; fn sub(self, other: Self) -> Self { Self::new(self.x - other.x, self.y - other.y) } }
impl Mul<f32> for Vec2 { type Output = Self; fn mul(self, rhs: f32) -> Self { Self::new(self.x * rhs, self.y * rhs) } }
impl Div<f32> for Vec2 { type Output = Self; fn div(self, rhs: f32) -> Self { Self::new(self.x / rhs, self.y / rhs) } }
impl AddAssign for Vec2 { fn add_assign(&mut self, other: Self) { self.x += other.x; self.y += other.y; } }
impl SubAssign for Vec2 { fn sub_assign(&mut self, other: Self) { self.x -= other.x; self.y -= other.y; } }

impl From<[f32; 2]> for Vec2 { fn from(v: [f32; 2]) -> Self { Self::new(v[0], v[1]) } }
impl From<(f32, f32)> for Vec2 { fn from(v: (f32, f32)) -> Self { Self::new(v.0, v.1) } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_ops() {
        let v1 = Vec2::new(10.0, 5.0);
        let v2 = Vec2::new(2.0, 3.0);
        
        let add = v1 + v2;
        assert_eq!(add.x, 12.0);
        assert_eq!(add.y, 8.0);

        let sub = v1 - v2;
        assert_eq!(sub.x, 8.0);
        assert_eq!(sub.y, 2.0);
    }

    #[test]
    fn test_vec2_length() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_vec2_normalize() {
        let v = Vec2::new(10.0, 0.0);
        let n = v.normalize();
        assert_eq!(n.x, 1.0);
        assert_eq!(n.y, 0.0);
    }
}
