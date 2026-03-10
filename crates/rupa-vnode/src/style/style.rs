use std::collections::HashMap;
use taffy::prelude::*;
use crate::spacing::Spacing;
use crate::border::{Border, Rounding, Outline};
use crate::background::Background;
use crate::color::Color;
use crate::layout::{Layout as RupaLayout, Display as RupaDisplay, Position as RupaPosition};
use crate::flex::{Flex as RupaFlex, FlexDirection as RupaFlexDirection, AlignItems as RupaAlignItems, JustifyContent as RupaJustifyContent};
use crate::grid::Grid as RupaGrid;
use crate::sizing::Sizing as RupaSizing;
use crate::typography::TypographyStyle;
use crate::interactivity::Interactivity;
use crate::effects::Shadow;
use crate::filters::Filter;
use crate::motion::Motion;
use crate::types::Breakpoint;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Style {
    pub layout: RupaLayout,
    pub flex: RupaFlex,
    pub grid: RupaGrid,
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

fn length(v: f32) -> LengthPercentage {
    LengthPercentage::Length(v)
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
    pub fn w_full(mut self) -> Self { self.sizing.width = Some(100.0); self }
    pub fn h_full(mut self) -> Self { self.sizing.height = Some(100.0); self }

    pub fn bg(mut self, color: Color) -> Self { self.background.color = Some(color); self }
    pub fn rounded(mut self, val: f32) -> Self { self.rounding = Rounding::all(val); self }

    pub fn to_taffy(&self) -> taffy::style::Style {
        let mut s = taffy::style::Style::default();
        
        s.display = match self.layout.display {
            RupaDisplay::Flex => Display::Flex,
            RupaDisplay::Grid => Display::Grid,
            RupaDisplay::None => Display::None,
            _ => Display::Block,
        };

        s.position = match self.layout.position {
            RupaPosition::Relative => Position::Relative,
            RupaPosition::Absolute => Position::Absolute,
            _ => Position::Relative, 
        };

        s.padding = Rect {
            left: length(self.padding.left),
            right: length(self.padding.right),
            top: length(self.padding.top),
            bottom: length(self.padding.bottom),
        };

        s.margin = Rect {
            left: length(self.margin.left).into(),
            right: length(self.margin.right).into(),
            top: length(self.margin.top).into(),
            bottom: length(self.margin.bottom).into(),
        };

        if let Some(w) = self.sizing.width { s.size.width = length(w).into(); }
        if let Some(h) = self.sizing.height { s.size.height = length(h).into(); }

        s.flex_direction = match self.flex.flex_direction {
            RupaFlexDirection::Row => FlexDirection::Row,
            RupaFlexDirection::Col => FlexDirection::Column,
            RupaFlexDirection::RowReverse => FlexDirection::RowReverse,
            RupaFlexDirection::ColReverse => FlexDirection::ColumnReverse,
        };

        if let Some(ref align) = self.flex.align_items {
            s.align_items = Some(match align {
                RupaAlignItems::FlexStart => AlignItems::FlexStart,
                RupaAlignItems::Center => AlignItems::Center,
                RupaAlignItems::FlexEnd => AlignItems::FlexEnd,
                RupaAlignItems::Stretch => AlignItems::Stretch,
                RupaAlignItems::Baseline => AlignItems::Baseline,
            });
        }

        if let Some(ref justify) = self.flex.justify_content {
            s.justify_content = Some(match justify {
                RupaJustifyContent::FlexStart => JustifyContent::FlexStart,
                RupaJustifyContent::Center => JustifyContent::Center,
                RupaJustifyContent::FlexEnd => JustifyContent::FlexEnd,
                RupaJustifyContent::SpaceBetween => JustifyContent::SpaceBetween,
                RupaJustifyContent::SpaceAround => JustifyContent::SpaceAround,
                RupaJustifyContent::SpaceEvenly => JustifyContent::SpaceEvenly,
            });
        }

        s
    }
}
