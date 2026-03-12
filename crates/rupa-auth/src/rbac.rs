use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// A specific capability or access right (e.g., "post:write", "admin:access").
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Permission(pub String);

/// A collection of permissions assigned to a user role.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Role {
    /// The unique name of the role (e.g., "editor", "admin").
    pub name: String,
    /// List of permissions granted to this role.
    pub permissions: Vec<Permission>,
}

/// A Port for checking authorization levels and permissions.
pub trait Provider: Send + Sync {
    /// Checks if a user has a specific permission.
    fn has_permission(&self, user_id: &str, permission: &Permission) -> bool;
    /// Checks if a user is assigned a specific role.
    fn has_role(&self, user_id: &str, role: &str) -> bool;
    /// Returns all roles assigned to a user.
    fn get_roles(&self, user_id: &str) -> Vec<Role>;
}

/// A foundational implementation of the RBAC Provider for basic application needs.
pub struct SimpleProvider {
    user_roles: HashMap<String, Vec<Role>>,
}

impl SimpleProvider {
    pub fn new() -> Self {
        Self { user_roles: HashMap::new() }
    }

    pub fn assign_role(&mut self, user_id: impl Into<String>, role: Role) {
        self.user_roles.entry(user_id.into()).or_default().push(role);
    }
}

impl Provider for SimpleProvider {
    fn has_permission(&self, user_id: &str, permission: &Permission) -> bool {
        self.user_roles.get(user_id)
            .map(|roles| roles.iter().any(|r| r.permissions.contains(permission)))
            .unwrap_or(false)
    }

    fn has_role(&self, user_id: &str, role: &str) -> bool {
        self.user_roles.get(user_id)
            .map(|roles| roles.iter().any(|r| r.name == role))
            .unwrap_or(false)
    }

    fn get_roles(&self, user_id: &str) -> Vec<Role> {
        self.user_roles.get(user_id).cloned().unwrap_or_default()
    }
}
