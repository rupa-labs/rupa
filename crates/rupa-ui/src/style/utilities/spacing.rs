#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

use taffy::prelude::Rect;
use taffy::style::LengthPercentage;

impl Spacing {
    pub fn all(val: f32) -> Self {
        Self { top: val, right: val, bottom: val, left: val }
    }

    pub fn to_taffy(&self) -> Rect<LengthPercentage> {
        Rect {
            left: LengthPercentage::Length(self.left),
            right: LengthPercentage::Length(self.right),
            top: LengthPercentage::Length(self.top),
            bottom: LengthPercentage::Length(self.bottom),
        }
    }
}

pub trait IntoSpacing {
    fn into_spacing(self) -> Spacing;
}

impl IntoSpacing for f32 {
    fn into_spacing(self) -> Spacing {
        Spacing::all(self)
    }
}
