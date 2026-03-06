use std::collections::HashMap;
use taffy::prelude::*;
use super::spacing::{Spacing};
use super::border::{Border, Rounding, Outline};
use super::background::{Background};
use super::color::Color;
use super::layout::{Layout as RupaLayout, Display as RupaDisplay, Position as RupaPosition};
use super::flex::{Flex as RupaFlex, FlexDirection as RupaFlexDirection, AlignItems as RupaAlignItems, JustifyContent as RupaJustifyContent};
use super::grid::Grid as RupaGrid;
use super::sizing::Sizing as RupaSizing;
use super::typography::Typography;
use super::effects::Shadow;
use super::filters::Filter;
use crate::style::modifiers::animation::Motion;

#[derive(Clone, Debug, Default, PartialEq)]
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
    pub typography: Typography,
    pub shadow: Option<Shadow>,
    pub filter: Option<Filter>,
    pub motion: Option<Motion>,
    pub is_group: bool,
    pub group_hover: Option<Box<Style>>,
    pub variants: HashMap<String, Style>,
}

impl Style {
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
            _ => Position::Relative, // Default to Relative for UI elements in Taffy if not Absolute
        };

        s.padding = Rect {
            left: length(self.padding.left),
            right: length(self.padding.right),
            top: length(self.padding.top),
            bottom: length(self.padding.bottom),
        };

        s.margin = Rect {
            left: length(self.margin.left),
            right: length(self.margin.right),
            top: length(self.margin.top),
            bottom: length(self.margin.bottom),
        };

        if let Some(w) = self.sizing.width { s.size.width = length(w); }
        if let Some(h) = self.sizing.height { s.size.height = length(h); }

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

    pub fn bg(mut self, color: Color) -> Self { self.background.color = Some(color); self }
    pub fn rounded(mut self, val: f32) -> Self { self.rounding = Rounding::all(val); self }
}
