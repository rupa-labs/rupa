# Module: Session Lifecycle (`crates/rupa-auth/src/session.rs`) ⏳

This module manages the active state of an authenticated user.

---

## 🧩 `struct Session`
A reactive state container for the current authentication token.

- **Validation**: Checks token expiry and signatures.
- **Refresh**: Handles automatic token renewal (e.g., OAuth2 refresh tokens).
- **Reactivity**: Exposes `is_authenticated` as a global Signal.
