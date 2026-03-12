use taffy::prelude::*;
use rupa_vnode::VNode;
use crate::renderer::TextMeasurer;
use super::SceneNode;

/// The mathematical engine for resolving UI layouts from a VNode tree.
pub struct LayoutEngine {
    pub taffy: TaffyTree<()>,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            taffy: TaffyTree::new(),
        }
    }

    /// Recursively builds the Taffy tree from a VNode tree and computes the final layout.
    pub fn compute(&mut self, root: &VNode, measurer: &dyn TextMeasurer, width: f32, height: f32) -> SceneNode {
        // Clear previous tree state for now (Future: partial updates)
        self.taffy.clear();
        
        let root_node = self.build_taffy_node(root, measurer);
        
        self.taffy.compute_layout(
            root_node, 
            Size { 
                width: AvailableSpace::Definite(width), 
                height: AvailableSpace::Definite(height) 
            }
        ).expect("Layout calculation failed");

        SceneNode::from(root_node)
    }

    fn build_taffy_node(&mut self, node: &VNode, _measurer: &dyn TextMeasurer) -> NodeId {
        match node {
            VNode::Element(el) => {
                let style = el.style.to_taffy();
                let children: Vec<NodeId> = el.children.iter()
                    .map(|child| self.build_taffy_node(child, _measurer))
                    .collect();
                
                self.taffy.new_with_children(style, &children).unwrap()
            }
            VNode::Text(_) => {
                // Future: Text measurement logic integration
                self.taffy.new_leaf(Style::default()).unwrap()
            }
            VNode::Fragment(children) => {
                // Fragments are transparent containers in layout
                let children: Vec<NodeId> = children.iter()
                    .map(|child| self.build_taffy_node(child, _measurer))
                    .collect();
                self.taffy.new_with_children(Style::default(), &children).unwrap()
            }
            _ => self.taffy.new_leaf(Style::default()).unwrap(),
        }
    }
}
