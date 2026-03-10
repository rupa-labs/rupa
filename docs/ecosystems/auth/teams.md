# Module: Team Management (`crates/rupa-auth/src/teams.rs`) 👥

This module provides support for multi-user organizations and shared resources.

---

## 🧩 `struct Team`
Represents a collection of users with shared access.

- **Membership**: Tracks roles within a team (e.g., "Owner", "Member").
- **Multi-Tenancy**: Allows a single user to switch between multiple teams reactively.
- **Resource Ownership**: Logic for checking if a team owns a specific data object.
