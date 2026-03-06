use std::collections::HashMap;
use taffy::prelude::*;
use super::spacing::{Spacing};
use super::border::{Border, Rounding, Outline};
use super::background::{Background};
use super::color::Color;
use super::layout::{Layout as RupaLayout, Display as RupaDisplay, Position as RupaPosition, Overflow as RupaOverflow};
use super::flex::{Flex as RupaFlex, FlexDirection as RupaFlexDirection, FlexWrap as RupaFlexWrap, AlignItems as RupaAlignItems, JustifyContent as RupaJustifyContent};
use super::grid::Grid as RupaGrid;
use super::sizing::Sizing as RupaSizing;
use super::typography::Typography;
use super::effects::Effects;
use super::filters::FilterStack;
use super::table::Table;
use super::animation::Motion;
use super::transform::Transform;
use super::interactivity::Interactivity;
use super::svg::Svg;
use super::accessibility::Accessibility;
use super::modifiers::StyleModifier;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style {
    pub layout: RupaLayout,
    pub flex: RupaFlex,
    pub grid: RupaGrid,
    pub sizing: RupaSizing,
    pub typography: Typography,
    pub background: Background,
    pub border: Border,
    pub outline: Outline,
    pub rounding: Rounding,
    pub margin: Spacing,
    pub padding: Spacing,
    pub effects: Effects,
    pub filters: FilterStack,
    pub table: Table,
    pub motion: Motion,
    pub transform: Transform,
    pub interactivity: Interactivity,
    pub svg: Svg,
    pub accessibility: Accessibility,
    pub custom: HashMap<String, String>,
    pub hover: Option<Box<Style>>,
    pub focus: Option<Box<Style>>,
    pub active: Option<Box<Style>>,
    pub disabled: Option<Box<Style>>,
    pub group_hover: Option<Box<Style>>,
    pub is_group: bool,
}

impl Style {
    pub fn new() -> Self { Self::default() }

    pub fn to_taffy(&self) -> taffy::style::Style {
        let mut t = taffy::style::Style::default();
        t.display = match self.layout.display {
            RupaDisplay::Flex => taffy::style::Display::Flex,
            RupaDisplay::Grid => taffy::style::Display::Grid,
            RupaDisplay::None => taffy::style::Display::None,
            _ => taffy::style::Display::Block,
        };
        t.position = match self.layout.position {
            RupaPosition::Relative => taffy::style::Position::Relative,
            RupaPosition::Absolute => taffy::style::Position::Absolute,
            _ => taffy::style::Position::Relative,
        };
        t.flex_direction = match self.flex.flex_direction {
            RupaFlexDirection::Row => taffy::style::FlexDirection::Row,
            RupaFlexDirection::Col => taffy::style::FlexDirection::Column,
            RupaFlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
            RupaFlexDirection::ColReverse => taffy::style::FlexDirection::ColumnReverse,
        };
        t.flex_wrap = match self.flex.flex_wrap {
            RupaFlexWrap::Wrap => taffy::style::FlexWrap::Wrap,
            RupaFlexWrap::WrapReverse => taffy::style::FlexWrap::WrapReverse,
            _ => taffy::style::FlexWrap::NoWrap,
        };
        t.flex_grow = self.flex.flex_grow;
        t.flex_shrink = self.flex.flex_shrink;
        if let Some(basis) = self.flex.flex_basis { t.flex_basis = length(basis); }
        if let Some(ref align) = self.flex.align_items {
            t.align_items = Some(match align {
                RupaAlignItems::Center => taffy::style::AlignItems::Center,
                RupaAlignItems::FlexStart => taffy::style::AlignItems::FlexStart,
                RupaAlignItems::FlexEnd => taffy::style::AlignItems::FlexEnd,
                RupaAlignItems::Stretch => taffy::style::AlignItems::Stretch,
                RupaAlignItems::Baseline => taffy::style::AlignItems::Baseline,
            });
        }
        if let Some(ref justify) = self.flex.justify_content {
            t.justify_content = Some(match justify {
                RupaJustifyContent::Center => taffy::style::JustifyContent::Center,
                RupaJustifyContent::FlexStart => taffy::style::JustifyContent::FlexStart,
                RupaJustifyContent::FlexEnd => taffy::style::JustifyContent::FlexEnd,
                RupaJustifyContent::SpaceBetween => taffy::style::JustifyContent::SpaceBetween,
                RupaJustifyContent::SpaceAround => taffy::style::JustifyContent::SpaceAround,
                RupaJustifyContent::SpaceEvenly => taffy::style::JustifyContent::SpaceEvenly,
            });
        }
        if let Some(gap) = self.flex.gap { t.gap = taffy::geometry::Size { width: length(gap), height: length(gap) }; }
        t.padding = taffy::geometry::Rect { left: length(self.padding.left), right: length(self.padding.right), top: length(self.padding.top), bottom: length(self.padding.bottom) };
        t.margin = taffy::geometry::Rect { left: length(self.margin.left), right: length(self.margin.right), top: length(self.margin.top), bottom: length(self.margin.bottom) };
        if let Some(w) = self.sizing.width { t.size.width = length(w); }
        if let Some(h) = self.sizing.height { t.size.height = length(h); }
        if let Some(min_w) = self.sizing.min_width { t.min_size.width = length(min_w); }
        if let Some(min_h) = self.sizing.min_height { t.min_size.height = length(min_h); }
        if let Some(max_w) = self.sizing.max_width { t.max_size.width = length(max_w); }
        if let Some(max_h) = self.sizing.max_height { t.max_size.height = length(max_h); }
        t.aspect_ratio = self.sizing.aspect_ratio;
        t.overflow.x = match self.layout.overflow_x { RupaOverflow::Hidden => taffy::style::Overflow::Hidden, RupaOverflow::Scroll => taffy::style::Overflow::Scroll, _ => taffy::style::Overflow::Visible };
        t.overflow.y = match self.layout.overflow_y { RupaOverflow::Hidden => taffy::style::Overflow::Hidden, RupaOverflow::Scroll => taffy::style::Overflow::Scroll, _ => taffy::style::Overflow::Visible };
        t
    }

    pub fn bg(mut self, color: impl Into<Color>) -> Self { self.background.color = Some(color.into()); self }
    pub fn rounded(mut self, val: f32) -> Self { self.rounding = Rounding::all(val); self }
    pub fn group(mut self) -> Self { self.is_group = true; self }
    pub fn group_hover(mut self, modifier: impl StyleModifier) -> Self {
        let mut s = self.group_hover.take().map(|b| *b).unwrap_or_default();
        modifier.apply(&mut s);
        self.group_hover = Some(Box::new(s));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_style_to_taffy_full() {
        let mut s = Style::new();
        s.padding = crate::utils::spacing::Spacing::all(10.0);
        let t = s.to_taffy();
        assert!(matches!(t.padding.left, LengthPercentage::Length(val) if val == 10.0));
    }
}
