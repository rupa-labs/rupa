use taffy::prelude::NodeId;

/// A platform-agnostic handle to a node in the Geometric Scene.
/// This wraps Taffy's NodeId to ensure Dependency Inversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SceneNode(pub(crate) NodeId);

impl SceneNode {
    /// Internal: Extract the raw Taffy ID. 
    /// Should only be used within Layer 3 or during rendering setup.
    pub fn raw(&self) -> NodeId {
        self.0
    }
}

impl From<NodeId> for SceneNode {
    fn from(id: NodeId) -> Self {
        Self(id)
    }
}
