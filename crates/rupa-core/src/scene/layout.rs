use rupa_vnode::{VNode, Style};
use rupa_base::Vec2;
use crate::renderer::TextMeasurer;
use super::SceneNode;
use serde::{Serialize, Deserialize};

/// # Rupa Layout Metrics 📏
/// 
/// Defines the physical translation layer for the layout engine. 
/// Distinguishes between Pixels (GUI) and Cells (TUI).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LayoutMode {
    /// High-precision pixel-based layout (Desktop/Web).
    Pixel,
    /// Grid-based character layout (Terminal). 
    /// Coordinates are rounded to the nearest integer (cell).
    Cell,
}

/// # Layout Solver Interface 🧩
/// 
/// The core contract for all layout engines in Rupa. 
/// This allows us to swap Taffy with a custom engine or any other solver 
/// without breaking the system.
pub trait LayoutSolver: Send + Sync {
    /// Resets the solver's internal state.
    fn reset(&mut self);

    /// Builds the layout tree from the given [VNode] and measures text.
    fn solve(&mut self, root: &VNode, measurer: &dyn TextMeasurer, width: f32, height: f32) -> SceneNode;

    /// Returns the physical position of a node, respecting the current [LayoutMode].
    fn get_position(&self, node: SceneNode, mode: LayoutMode) -> Vec2;

    /// Returns the physical size of a node, respecting the current [LayoutMode].
    fn get_size(&self, node: SceneNode, mode: LayoutMode) -> Vec2;
}

/// The high-level engine that orchestrates layout calculation. 
/// It uses a [LayoutSolver] to do the heavy lifting.
pub struct LayoutEngine {
    pub solver: Box<dyn LayoutSolver>,
    pub mode: LayoutMode,
}

impl LayoutEngine {
    pub fn new(solver: Box<dyn LayoutSolver>, mode: LayoutMode) -> Self {
        Self { solver, mode }
    }

    pub fn compute(&mut self, root: &VNode, measurer: &dyn TextMeasurer, width: f32, height: f32) -> SceneNode {
        self.solver.solve(root, measurer, width, height)
    }

    pub fn get_physical_position(&self, node: SceneNode) -> Vec2 {
        self.solver.get_position(node, self.mode)
    }

    pub fn get_physical_size(&self, node: SceneNode) -> Vec2 {
        self.solver.get_size(node, self.mode)
    }
}

// --- Taffy Adapter (The Current Implementation) ---

pub struct TaffySolver {
    tree: taffy::prelude::TaffyTree<()>,
}

impl TaffySolver {
    pub fn new() -> Self {
        Self { tree: taffy::prelude::TaffyTree::new() }
    }

    fn build_node(&mut self, node: &VNode, measurer: &dyn TextMeasurer) -> taffy::prelude::NodeId {
        match node {
            VNode::Element(el) => {
                let style = self.translate_style(&el.style);
                let children: Vec<taffy::prelude::NodeId> = el.children.iter()
                    .map(|child| self.build_node(child, measurer))
                    .collect();
                self.tree.new_with_children(style, &children).unwrap()
            }
            VNode::Text(_) => self.tree.new_leaf(taffy::prelude::Style::default()).unwrap(),
            VNode::Fragment(children) => {
                let children: Vec<taffy::prelude::NodeId> = children.iter()
                    .map(|child| self.build_node(child, measurer))
                    .collect();
                self.tree.new_with_children(taffy::prelude::Style::default(), &children).unwrap()
            }
            _ => self.tree.new_leaf(taffy::prelude::Style::default()).unwrap(),
        }
    }

    fn translate_style(&self, rupa: &Style) -> taffy::prelude::Style {
        use taffy::prelude::*;
        use rupa_vnode::layout::{Display, Position};
        use rupa_vnode::flex::{FlexDirection, AlignItems, JustifyContent, FlexWrap};
        use rupa_vnode::grid::GridAutoFlow;

        let mut s = taffy::prelude::Style::default();

        s.display = match rupa.layout.display {
            Display::Flex => taffy::prelude::Display::Flex,
            Display::Grid => taffy::prelude::Display::Grid,
            Display::None => taffy::prelude::Display::None,
            _ => taffy::prelude::Display::Block,
        };

        s.position = match rupa.layout.position {
            Position::Relative => taffy::prelude::Position::Relative,
            Position::Absolute => taffy::prelude::Position::Absolute,
            _ => taffy::prelude::Position::Relative,
        };

        s.flex_direction = match rupa.flex.flex_direction {
            FlexDirection::Row => taffy::prelude::FlexDirection::Row,
            FlexDirection::Col => taffy::prelude::FlexDirection::Column,
            FlexDirection::RowReverse => taffy::prelude::FlexDirection::RowReverse,
            FlexDirection::ColReverse => taffy::prelude::FlexDirection::ColumnReverse,
        };

        s.flex_wrap = match rupa.flex.flex_wrap {
            FlexWrap::NoWrap => taffy::prelude::FlexWrap::NoWrap,
            FlexWrap::Wrap => taffy::prelude::FlexWrap::Wrap,
            FlexWrap::WrapReverse => taffy::prelude::FlexWrap::WrapReverse,
        };

        if let Some(w) = rupa.sizing.width { 
            s.size.width = if w < 0.0 { Dimension::Percent(1.0) } else { LengthPercentage::Length(w).into() }; 
        }
        if let Some(h) = rupa.sizing.height { 
            s.size.height = if h < 0.0 { Dimension::Percent(1.0) } else { LengthPercentage::Length(h).into() }; 
        }

        s.gap = Size {
            width: LengthPercentage::Length(rupa.flex.gap.unwrap_or(0.0)),
            height: LengthPercentage::Length(rupa.flex.gap.unwrap_or(0.0)),
        };

        if let Some(ref align) = rupa.flex.align_items {
            s.align_items = Some(match align {
                AlignItems::FlexStart => taffy::prelude::AlignItems::FlexStart,
                AlignItems::Center => taffy::prelude::AlignItems::Center,
                AlignItems::FlexEnd => taffy::prelude::AlignItems::FlexEnd,
                AlignItems::Stretch => taffy::prelude::AlignItems::Stretch,
                AlignItems::Baseline => taffy::prelude::AlignItems::Baseline,
            });
        }

        if let Some(ref justify) = rupa.flex.justify_content {
            s.justify_content = Some(match justify {
                JustifyContent::FlexStart => taffy::prelude::JustifyContent::FlexStart,
                JustifyContent::Center => taffy::prelude::JustifyContent::Center,
                JustifyContent::FlexEnd => taffy::prelude::JustifyContent::FlexEnd,
                JustifyContent::SpaceBetween => taffy::prelude::JustifyContent::SpaceBetween,
                JustifyContent::SpaceAround => taffy::prelude::JustifyContent::SpaceAround,
                JustifyContent::SpaceEvenly => taffy::prelude::JustifyContent::SpaceEvenly,
            });
        }

        // Grid Mapping
        if s.display == taffy::prelude::Display::Grid {
            s.grid_auto_flow = match rupa.grid.auto_flow {
                GridAutoFlow::Row => taffy::prelude::GridAutoFlow::Row,
                GridAutoFlow::Column => taffy::prelude::GridAutoFlow::Column,
                GridAutoFlow::RowDense => taffy::prelude::GridAutoFlow::RowDense,
                GridAutoFlow::ColumnDense => taffy::prelude::GridAutoFlow::ColumnDense,
            };
            // ... (Template mapping would go here)
        }

        s
    }
}

impl LayoutSolver for TaffySolver {
    fn reset(&mut self) {
        self.tree.clear();
    }

    fn solve(&mut self, root: &VNode, measurer: &dyn TextMeasurer, width: f32, height: f32) -> SceneNode {
        self.reset();
        let root_id = self.build_node(root, measurer);
        self.tree.compute_layout(
            root_id,
            taffy::prelude::Size {
                width: taffy::prelude::AvailableSpace::Definite(width),
                height: taffy::prelude::AvailableSpace::Definite(height),
            }
        ).unwrap();
        SceneNode::from(root_id)
    }

    fn get_position(&self, node: SceneNode, mode: LayoutMode) -> Vec2 {
        let layout = self.tree.layout(node.raw()).unwrap();
        match mode {
            LayoutMode::Pixel => Vec2::new(layout.location.x, layout.location.y),
            LayoutMode::Cell => Vec2::new(layout.location.x.round(), layout.location.y.round()),
        }
    }

    fn get_size(&self, node: SceneNode, mode: LayoutMode) -> Vec2 {
        let layout = self.tree.layout(node.raw()).unwrap();
        match mode {
            LayoutMode::Pixel => Vec2::new(layout.size.width, layout.size.height),
            LayoutMode::Cell => Vec2::new(layout.size.width.round(), layout.size.height.round()),
        }
    }
}
