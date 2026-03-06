use taffy::prelude::*;
use std::collections::HashMap;
use crate::core::component::Component;

pub struct LayoutEngine {
    pub taffy: TaffyTree<()>,
    pub node_map: HashMap<String, NodeId>,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            taffy: TaffyTree::new(),
            node_map: HashMap::new(),
        }
    }

    pub fn compute(&mut self, root: &dyn Component, width: f32, height: f32) -> NodeId {
        // Phase 1: Sync Element Tree with Taffy Tree (Smart Update)
        // Instead of clear(), we let the components manage their own nodes.
        let root_node = root.layout(&mut self.taffy, None);
        
        // Phase 2: Compute Layout
        // Taffy is smart enough to only recompute what changed.
        self.taffy.compute_layout(
            root_node,
            Size {
                width: AvailableSpace::Definite(width),
                height: AvailableSpace::Definite(height),
            },
        ).unwrap();

        root_node
    }

    pub fn get_layout(&self, node: NodeId) -> Option<&Layout> {
        self.taffy.layout(node).ok()
    }
}
