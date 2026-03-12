use crate::Vec2;
use serde::{Serialize, Deserialize};

/// A rectangle representing layout bounds or a bounding box.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub origin: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            origin: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }

    pub fn from_origin_size(origin: Vec2, size: Vec2) -> Self {
        Self { origin, size }
    }

    pub fn x(&self) -> f32 { self.origin.x }
    pub fn y(&self) -> f32 { self.origin.y }
    pub fn width(&self) -> f32 { self.size.x }
    pub fn height(&self) -> f32 { self.size.y }

    pub fn left(&self) -> f32 { self.origin.x }
    pub fn right(&self) -> f32 { self.origin.x + self.size.x }
    pub fn top(&self) -> f32 { self.origin.y }
    pub fn bottom(&self) -> f32 { self.origin.y + self.size.y }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.origin.x + self.size.x * 0.5,
            self.origin.y + self.size.y * 0.5,
        )
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.left() && point.x <= self.right() &&
        point.y >= self.top() && point.y <= self.bottom()
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        !(self.right() < other.left() ||
          self.left() > other.right() ||
          self.bottom() < other.top() ||
          self.top() > other.bottom())
    }

    /// Returns a new Rect that is inset by the given amount on all sides.
    pub fn inset(&self, amount: f32) -> Self {
        Self::new(
            self.origin.x + amount,
            self.origin.y + amount,
            (self.size.x - amount * 2.0).max(0.0),
            (self.size.y - amount * 2.0).max(0.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_bounds() {
        let r = Rect::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(r.left(), 10.0);
        assert_eq!(r.right(), 110.0);
        assert_eq!(r.top(), 20.0);
        assert_eq!(r.bottom(), 70.0);
    }

    #[test]
    fn test_rect_contains() {
        let r = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert!(r.contains(Vec2::new(50.0, 50.0)));
        assert!(!r.contains(Vec2::new(150.0, 50.0)));
    }

    #[test]
    fn test_rect_intersects() {
        let r1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let r2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        let r3 = Rect::new(200.0, 200.0, 100.0, 100.0);
        
        assert!(r1.intersects(&r2));
        assert!(!r1.intersects(&r3));
    }
}
