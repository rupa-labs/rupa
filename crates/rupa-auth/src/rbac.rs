use serde::{Serialize, Deserialize};

/// Definitions for RBAC Port (Authorization).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Permission(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Role {
    pub name: String,
    pub permissions: Vec<Permission>,
}

/// A Port for checking authorization levels.
pub trait Provider: Send + Sync {
    fn has_permission(&self, user_id: &str, permission: &Permission) -> bool;
    fn has_role(&self, user_id: &str, role: &str) -> bool;
    fn get_roles(&self, user_id: &str) -> Vec<Role>;
}
