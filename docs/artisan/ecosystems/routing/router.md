# Module: Router Component (`crates/rupa-router/src/router.rs`) 🗺️

This module defines the high-level `Router` component used to manage view transitions based on the current application path.

---

## 🏗️ `struct Router`
A container component that conditionally renders its children.

- **Switch Logic**: Only the child `Route` matching the current path is rendered.
- **VNode Integration**: Works directly with the core reconciliation engine to perform smooth page transitions.
