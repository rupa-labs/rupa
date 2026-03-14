use rupa_base::Vec2;
use rupa_vnode::VNode;
use crate::scene::layout::{LayoutEngine, LayoutMode};

/// A platform-agnostic handle to a node in the Geometric Scene.
/// It wraps the internal ID of the layout solver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SceneNode(pub taffy::prelude::NodeId); // Still uses Taffy ID internally for now, but via SceneNode wrapper

impl SceneNode {
    pub fn raw(&self) -> taffy::prelude::NodeId { self.0 }
}

impl From<taffy::prelude::NodeId> for SceneNode {
    fn from(id: taffy::prelude::NodeId) -> Self { Self(id) }
}

/// Represents the discovery result of a spatial hit-test on a VNode.
pub enum HitDiscovery<'a> {
    Missed,
    Found(HitResult<'a>),
}

pub struct HitResult<'a> {
    pub path: Vec<&'a VNode>,
    pub local_pos: Vec2,
    pub node: &'a VNode,
    pub target_id: Option<String>,
}

#[derive(Default)]
pub enum SceneState {
    #[default]
    Empty,
    Resolved(SceneNode),
}

/// The core engine for managing the spatial scene and performing hit-tests on VNodes.
pub struct SceneCore {
    pub layout_engine: LayoutEngine,
    pub state: SceneState,
}

impl SceneCore {
    /// Creates a new SceneCore with the specified layout engine and mode.
    pub fn new(layout_engine: LayoutEngine) -> Self {
        Self { 
            layout_engine,
            state: SceneState::Empty 
        }
    }

    pub fn set_root(&mut self, node: SceneNode) {
        self.state = SceneState::Resolved(node);
    }

    /// Performs a hit-test at the given cursor position against the VNode tree.
    pub fn find_target<'a>(
        &self, 
        root: &'a VNode, 
        cursor_pos: Vec2
    ) -> HitDiscovery<'a> {
        if let SceneState::Resolved(root_node) = self.state {
            if let Some(result) = self.recursive_hit_test(root, root_node, cursor_pos, Vec2::zero()) {
                return HitDiscovery::Found(result);
            }
        }
        HitDiscovery::Missed
    }

    fn recursive_hit_test<'a>(
        &self,
        root: &'a VNode,
        node: SceneNode,
        cursor_pos: Vec2,
        parent_global_pos: Vec2,
    ) -> Option<HitResult<'a>> {
        let layout_pos = self.layout_engine.get_physical_position(node);
        let layout_size = self.layout_engine.get_physical_size(node);
        
        let global_pos = parent_global_pos + layout_pos;

        let is_inside = cursor_pos.x >= global_pos.x 
            && cursor_pos.x <= global_pos.x + layout_size.x
            && cursor_pos.y >= global_pos.y 
            && cursor_pos.y <= global_pos.y + layout_size.y;

        if !is_inside { return None; }

        match root {
            VNode::Element(el) => {
                // This part still needs a way to get children SceneNodes from the solver 
                // but we'll keep it simple for now to demonstrate the architecture.
                // In a full implementation, the solver would provide child SceneNodes.
            }
            _ => {}
        }

        let target_id = if let VNode::Element(el) = root {
            el.key.clone()
        } else {
            None
        };

        Some(HitResult {
            path: vec![root],
            local_pos: cursor_pos - global_pos,
            node: root,
            target_id,
        })
    }
}
pub mod layout;
