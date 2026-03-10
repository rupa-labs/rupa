use rupa_support::Vec2;
use crate::component::Component;
use taffy::prelude::{NodeId, TaffyTree};

/// A platform-agnostic handle to a node in the Geometric Scene.
/// This wraps Taffy's NodeId to ensure Dependency Inversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SceneNode(pub NodeId);

impl SceneNode {
    pub fn raw(&self) -> NodeId { self.0 }
}

impl From<NodeId> for SceneNode {
    fn from(id: NodeId) -> Self { Self(id) }
}

/// Represents the discovery result of a spatial hit-test.
pub enum HitDiscovery<'a> {
    Missed,
    Found(HitResult<'a>),
}

/// The result of a successful hit-test, containing the full ancestor path.
pub struct HitResult<'a> {
    pub path: Vec<&'a dyn Component>,
    pub local_pos: Vec2,
    pub component: &'a dyn Component,
}

/// Represents the current state of the geometric scene.
#[derive(Default)]
pub enum SceneState {
    #[default]
    Empty,
    Resolved(SceneNode),
}

use crate::scene::layout::LayoutEngine;

/// The core engine for managing the spatial scene and performing hit-tests.
/// It is decoupled from any specific renderer but uses Taffy for layout calculations.
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

    /// Finds a component by its ID and returns its hit result.
    pub fn find_by_id<'a>(
        &self, 
        root: &'a dyn Component, 
        taffy: &TaffyTree<()>,
        target_id: &str
    ) -> Option<HitResult<'a>> {
        if let SceneState::Resolved(root_node) = self.state {
            let mut path = Vec::new();
            if self.recursive_find_id(root, taffy, root_node.raw(), target_id, Vec2::zero(), &mut path) {
                return Some(HitResult {
                    path,
                    local_pos: Vec2::zero(), // Not used for ID-based lookup usually
                    component: root, // This will be the actual target due to path recursion
                });
            }
        }
        None
    }

    fn recursive_find_id<'a>(
        &self,
        root: &'a dyn Component,
        taffy: &TaffyTree<()>,
        node: NodeId,
        target_id: &str,
        parent_global_pos: Vec2,
        path: &mut Vec<&'a dyn Component>,
    ) -> bool {
        let layout = taffy.layout(node).unwrap();
        let global_pos = parent_global_pos + Vec2::new(layout.location.x, layout.location.y);

        if root.id() == target_id {
            path.push(root);
            return true;
        }

        let children = root.children();
        let taffy_children = taffy.children(node).unwrap();

        for (i, child) in children.iter().enumerate() {
            if let Some(child_node) = taffy_children.get(i) {
                if self.recursive_find_id(*child, taffy, *child_node, target_id, global_pos, path) {
                    path.push(root);
                    return true;
                }
            }
        }

        false
    }

    /// Performs a hit-test at the given cursor position.
    pub fn find_target<'a>(
        &self, 
        root: &'a dyn Component, 
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
        root: &'a dyn Component,
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

        let children = root.children();
        let taffy_children = taffy.children(node).unwrap();

        // Check children in reverse (topmost first)
        for (i, child) in children.iter().enumerate().rev() {
            if let Some(child_node) = taffy_children.get(i) {
                if let Some(mut result) = self.recursive_hit_test(*child, taffy, *child_node, cursor_pos, global_pos) {
                    result.path.push(root);
                    return Some(result);
                }
            }
        }

        Some(HitResult {
            path: vec![root],
            local_pos: cursor_pos - global_pos,
            component: root,
        })
    }
}
pub mod layout;
