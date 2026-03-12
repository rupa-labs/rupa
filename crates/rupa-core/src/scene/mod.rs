use rupa_base::Vec2;
use rupa_vnode::VNode;
use taffy::prelude::{NodeId, TaffyTree};

/// A platform-agnostic handle to a node in the Geometric Scene.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SceneNode(pub NodeId);

impl SceneNode {
    pub fn raw(&self) -> NodeId { self.0 }
}

impl From<NodeId> for SceneNode {
    fn from(id: NodeId) -> Self { Self(id) }
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
}

#[derive(Default)]
pub enum SceneState {
    #[default]
    Empty,
    Resolved(SceneNode),
}

use crate::scene::layout::LayoutEngine;

/// The core engine for managing the spatial scene and performing hit-tests on VNodes.
pub struct SceneCore {
    pub layout_engine: LayoutEngine,
    pub state: SceneState,
}

impl SceneCore {
    pub fn new() -> Self {
        Self { 
            layout_engine: LayoutEngine::new(),
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
        taffy: &TaffyTree<()>,
        cursor_pos: Vec2
    ) -> HitDiscovery<'a> {
        if let SceneState::Resolved(root_node) = self.state {
            if let Some(result) = self.recursive_hit_test(root, taffy, root_node.raw(), cursor_pos, Vec2::zero()) {
                return HitDiscovery::Found(result);
            }
        }
        HitDiscovery::Missed
    }

    fn recursive_hit_test<'a>(
        &self,
        root: &'a VNode,
        taffy: &TaffyTree<()>,
        node: NodeId,
        cursor_pos: Vec2,
        parent_global_pos: Vec2,
    ) -> Option<HitResult<'a>> {
        let layout = taffy.layout(node).unwrap();
        let global_pos = parent_global_pos + Vec2::new(layout.location.x, layout.location.y);

        let is_inside = cursor_pos.x >= global_pos.x 
            && cursor_pos.x <= global_pos.x + layout.size.width
            && cursor_pos.y >= global_pos.y 
            && cursor_pos.y <= global_pos.y + layout.size.height;

        if !is_inside { return None; }

        match root {
            VNode::Element(el) => {
                let taffy_children = taffy.children(node).unwrap();
                for (i, child) in el.children.iter().enumerate().rev() {
                    if let Some(child_node) = taffy_children.get(i) {
                        if let Some(mut result) = self.recursive_hit_test(child, taffy, *child_node, cursor_pos, global_pos) {
                            result.path.push(root);
                            return Some(result);
                        }
                    }
                }
            }
            VNode::Fragment(children) => {
                let taffy_children = taffy.children(node).unwrap();
                for (i, child) in children.iter().enumerate().rev() {
                    if let Some(child_node) = taffy_children.get(i) {
                        if let Some(mut result) = self.recursive_hit_test(child, taffy, *child_node, cursor_pos, global_pos) {
                            result.path.push(root);
                            return Some(result);
                        }
                    }
                }
            }
            _ => {}
        }

        Some(HitResult {
            path: vec![root],
            local_pos: cursor_pos - global_pos,
            node: root,
        })
    }
}
pub mod layout;
