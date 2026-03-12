use rupa_base::Vec2;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PathSegment {
    MoveTo(Vec2),
    LineTo(Vec2),
    CurveTo(Vec2, Vec2, Vec2),
    QuadTo(Vec2, Vec2),
    ArcTo { radius: Vec2, rotation: f32, large_arc: bool, sweep: bool, end: Vec2 },
    Close,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PathData {
    pub segments: Vec<PathSegment>,
}

impl PathData {
    pub fn new() -> Self { Self::default() }

    pub fn move_to(mut self, p: impl Into<Vec2>) -> Self {
        self.segments.push(PathSegment::MoveTo(p.into()));
        self
    }

    pub fn line_to(mut self, p: impl Into<Vec2>) -> Self {
        self.segments.push(PathSegment::LineTo(p.into()));
        self
    }

    pub fn close(mut self) -> Self {
        self.segments.push(PathSegment::Close);
        self
    }

    pub fn to_svg_string(&self) -> String {
        let mut d = String::new();
        for seg in &self.segments {
            match seg {
                PathSegment::MoveTo(p) => d.push_str(&format!("M {} {} ", p.x, p.y)),
                PathSegment::LineTo(p) => d.push_str(&format!("L {} {} ", p.x, p.y)),
                PathSegment::CurveTo(c1, c2, end) => d.push_str(&format!("C {} {}, {} {}, {} {} ", c1.x, c1.y, c2.x, c2.y, end.x, end.y)),
                PathSegment::QuadTo(c, end) => d.push_str(&format!("Q {} {}, {} {} ", c.x, c.y, end.x, end.y)),
                PathSegment::ArcTo { radius, rotation, large_arc, sweep, end } => {
                    d.push_str(&format!("A {} {} {} {} {} {} {} ", radius.x, radius.y, rotation, *large_arc as u8, *sweep as u8, end.x, end.y));
                }
                PathSegment::Close => d.push_str("Z "),
            }
        }
        d.trim().to_string()
    }
}
