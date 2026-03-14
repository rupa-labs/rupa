use std::collections::HashMap;
use crate::spacing::Spacing;
use crate::border::{Border, Rounding, Outline};
use crate::background::Background;
use crate::color::Color;
use crate::layout::{Layout as RupaLayout};
use crate::flex::{Flex as RupaFlex};
use crate::grid::{Grid as RupaGrid, GridPlacement as RupaGridPlacement};
use crate::sizing::Sizing as RupaSizing;
use crate::typography::TypographyStyle;
use crate::interactivity::Interactivity;
use crate::effects::Shadow;
use crate::filters::Filter;
use crate::motion::Motion;
use crate::types::Breakpoint;
use serde::{Serialize, Deserialize};

/// # Rupa Style 🎨
/// 
/// The platform-agnostic visual DNA of an element. 
/// It contains layout rules, colors, borders, and effects but is 
/// entirely decoupled from any specific layout or rendering engine.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Style {
    pub layout: RupaLayout,
    pub flex: RupaFlex,
    pub grid: RupaGrid,
    pub grid_item: RupaGridPlacement,
    pub sizing: RupaSizing,
    pub spacing: Spacing,
    pub padding: Spacing,
    pub margin: Spacing,
    pub background: Background,
    pub border: Border,
    pub rounding: Rounding,
    pub outline: Outline,
    pub typography: TypographyStyle,
    pub interactivity: Interactivity,
    pub shadow: Option<Shadow>,
    pub filter: Option<Filter>,
    pub motion: Option<Motion>,
    pub is_group: bool,
    pub group_hover: Option<Box<Style>>,
    pub responsive: HashMap<Breakpoint, Box<Style>>,
    pub variants: HashMap<String, Style>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }

    pub fn p(mut self, val: f32) -> Self { self.padding = Spacing::all(val); self }
    pub fn px(mut self, val: f32) -> Self { self.padding.left = val; self.padding.right = val; self }
    pub fn py(mut self, val: f32) -> Self { self.padding.top = val; self.padding.bottom = val; self }
    
    pub fn m(mut self, val: f32) -> Self { self.margin = Spacing::all(val); self }
    pub fn mx(mut self, val: f32) -> Self { self.margin.left = val; self.margin.right = val; self }
    pub fn my(mut self, val: f32) -> Self { self.margin.top = val; self.margin.bottom = val; self }

    pub fn w(mut self, val: f32) -> Self { self.sizing.width = Some(val); self }
    pub fn h(mut self, val: f32) -> Self { self.sizing.height = Some(val); self }
    
    pub fn w_full(mut self) -> Self { self.sizing.width = Some(-1.0); self } 
    pub fn h_full(mut self) -> Self { self.sizing.height = Some(-1.0); self }

    pub fn bg(mut self, color: Color) -> Self { self.background.color = Some(color); self }
    pub fn rounded(mut self, val: f32) -> Self { self.rounding = Rounding::all(val); self }
}
