# Module: Authentication & Identity (`crates/rupa-auth`) 🔐

The `rupa-auth` crate handles user identity, session management, and multi-tenant team structures.

---

## 1. Core Philosophy: Secure-by-Default Identity
Authentication is integrated into the reactive graph, allowing UI components to automatically react to login/logout states.

## 2. Module Structure (1:1 Mapping)
- `identity.rs`: The `User` struct and core authentication traits.
- `session.rs`: Logic for token management (JWT, Cookie) and persistent login.
- `rbac.rs`: Role-Based Access Control system for fine-grained permissions.
- `teams.rs`: Support for multi-user organizations and shared resource ownership.
- `utils.rs`: Cryptographic helpers for password hashing and secure token generation.
