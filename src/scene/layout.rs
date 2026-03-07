use taffy::prelude::*;
use crate::core::component::Component;
use crate::renderer::TextMeasurer;
use super::node::SceneNode;

/// The mathematical engine for resolving UI layouts using Taffy.
pub struct LayoutEngine {
    pub taffy: TaffyTree<()>,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            taffy: TaffyTree::new(),
        }
    }

    /// Recursively builds the Taffy tree and computes the final layout.
    pub fn compute(&mut self, root: &dyn Component, measurer: &dyn TextMeasurer, width: f32, height: f32) -> SceneNode {
        let root_node = root.layout(&mut self.taffy, measurer, None);
        
        self.taffy.compute_layout(
            root_node, 
            Size { 
                width: AvailableSpace::Definite(width), 
                height: AvailableSpace::Definite(height) 
            }
        ).expect("Layout calculation failed");

        SceneNode::from(root_node)
    }
}
