# Module: Role-Based Access Control (`crates/rupa-auth/src/rbac.rs`) 🛡️

This module implements the authorization logic for the framework.

---

## 🏗️ Logic
- **Permissions**: Defines atomic actions (e.g., "edit_post", "delete_user").
- **Roles**: Groups of permissions (e.g., "Admin", "Editor", "Guest").
- **Guards**: Logic to prevent rendering of UI components if the user lacks a required permission.
