# Module: User Identity (`crates/rupa-auth/src/identity.rs`) 👤

This module defines the core user data structures and authentication traits.

---

## 🧩 `struct User`
A serializable object representing an authenticated person.

- **Fields**: `id`, `email`, `username`, `metadata`.
- **Persistence**: Automatically stored in `rupa-store` upon successful login.
