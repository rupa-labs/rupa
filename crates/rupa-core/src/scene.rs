use taffy::prelude::NodeId;

/// A platform-agnostic handle to a node in the Geometric Scene.
/// This wraps Taffy's NodeId to ensure Dependency Inversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SceneNode(pub NodeId);

impl SceneNode {
    /// Internal: Extract the raw Taffy ID. 
    pub fn raw(&self) -> NodeId {
        self.0
    }
}

impl From<NodeId> for SceneNode {
    fn from(id: NodeId) -> Self {
        Self(id)
    }
}
