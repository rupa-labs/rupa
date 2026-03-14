use serde::{Serialize, Deserialize};
use crate::types::Unit;

/// # Rupa Spacing 📏
/// 
/// Defines the layout spacing (Padding, Margin) for a UI element.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Spacing {
    pub top: Unit,
    pub right: Unit,
    pub bottom: Unit,
    pub left: Unit,
}

impl Spacing {
    pub fn new(top: impl Into<Unit>, right: impl Into<Unit>, bottom: impl Into<Unit>, left: impl Into<Unit>) -> Spacing {
        Self { 
            top: top.into(), 
            right: right.into(), 
            bottom: bottom.into(), 
            left: left.into() 
        }
    }

    pub fn all(val: impl Into<Unit>) -> Self {
        let u = val.into();
        Self { top: u.clone(), right: u.clone(), bottom: u.clone(), left: u }
    }

    pub fn x(mut self, val: impl Into<Unit>) -> Self {
        let u = val.into();
        self.left = u.clone();
        self.right = u;
        self
    }

    pub fn y(mut self, val: impl Into<Unit>) -> Self {
        let u = val.into();
        self.top = u.clone();
        self.bottom = u;
        self
    }
}
