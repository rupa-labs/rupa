pub mod layout;
pub mod node;

use crate::core::component::Component;
use crate::support::vector::Vec2;
use crate::renderer::TextMeasurer;
use self::layout::LayoutEngine;
pub use self::node::SceneNode;

/// Represents the current state of the geometric scene.
pub enum SceneState {
    Empty,
    Resolved(SceneNode),
}

impl SceneState {
    pub fn raw(&self) -> taffy::prelude::NodeId {
        match self {
            SceneState::Resolved(node) => node.raw(),
            SceneState::Empty => panic!("SceneState::raw() called on Empty state"),
        }
    }
}

pub enum HitDiscovery<'a> {
    Missed,
    Found(HitResult<'a>),
}

pub struct HitResult<'a> {
    pub path: Vec<&'a dyn Component>,
    pub local_pos: Vec2,
    pub component: &'a dyn Component,
}

pub struct SceneCore {
    pub layout_engine: LayoutEngine,
    pub state: SceneState,
}

impl SceneCore {
    pub fn new() -> Self {
        Self {
            layout_engine: LayoutEngine::new(),
            state: SceneState::Empty,
        }
    }

    /// Finds a component by its ID and returns its global position.
    /// This is crucial for mouse capture when the cursor moves outside the component bounds.
    pub fn find_by_id<'a>(
        &self, 
        root: &'a dyn Component, 
        target_id: &str
    ) -> Option<HitResult<'a>> {
        if let SceneState::Resolved(root_node) = self.state {
            return self.recursive_find_id(root, root_node.raw(), target_id, Vec2::zero());
        }
        None
    }

    fn recursive_find_id<'a>(
        &self,
        root: &'a dyn Component,
        node: taffy::prelude::NodeId,
        target_id: &str,
        parent_global_pos: Vec2,
    ) -> Option<HitResult<'a>> {
        let layout = self.layout_engine.taffy.layout(node).ok()?;
        let global_pos = parent_global_pos + Vec2::new(layout.location.x, layout.location.y);

        if root.id() == target_id {
            return Some(HitResult {
                path: vec![root],
                local_pos: global_pos, // Return global_pos as local_pos for now
                component: root,
            });
        }

        let children = root.children();
        let taffy_children = self.layout_engine.taffy.children(node).ok()?;

        for (i, child) in children.iter().enumerate() {
            if let Some(child_node) = taffy_children.get(i) {
                if let Some(mut result) = self.recursive_find_id(*child, *child_node, target_id, global_pos) {
                    result.path.push(root);
                    return Some(result);
                }
            }
        }

        None
    }

    /// Resolves the spatial coordinates for the entire tree.
    pub fn resolve(&mut self, root: &dyn Component, measurer: &dyn TextMeasurer, width: f32, height: f32) -> SceneNode {
        let node = self.layout_engine.compute(root, measurer, width, height);
        let scene_node = SceneNode::from(node);
        self.state = SceneState::Resolved(scene_node);
        scene_node
    }

    pub fn find_target<'a>(
        &self, 
        root: &'a dyn Component, 
        cursor_pos: Vec2
    ) -> HitDiscovery<'a> {
        if let SceneState::Resolved(root_node) = self.state {
            if let Some(result) = self.recursive_hit_test(root, root_node.raw(), cursor_pos, Vec2::zero()) {
                return HitDiscovery::Found(result);
            }
        }
        HitDiscovery::Missed
    }

    fn recursive_hit_test<'a>(
        &self,
        root: &'a dyn Component,
        node: taffy::prelude::NodeId,
        cursor_pos: Vec2,
        parent_global_pos: Vec2,
    ) -> Option<HitResult<'a>> {
        let layout = self.layout_engine.taffy.layout(node).ok()?;
        let global_pos = parent_global_pos + Vec2::new(layout.location.x, layout.location.y);

        let is_inside = cursor_pos.x >= global_pos.x 
            && cursor_pos.x <= global_pos.x + layout.size.width
            && cursor_pos.y >= global_pos.y 
            && cursor_pos.y <= global_pos.y + layout.size.height;

        if !is_inside { return None; }

        let children = root.children();
        let taffy_children = self.layout_engine.taffy.children(node).ok()?;

        for (i, child) in children.iter().enumerate().rev() {
            if let Some(child_node) = taffy_children.get(i) {
                if let Some(mut result) = self.recursive_hit_test(*child, *child_node, cursor_pos, global_pos) {
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
