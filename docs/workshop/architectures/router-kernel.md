# Router Kernel Architecture 🧭

The **Router Kernel** is a Tier 2 system that provides reactive, hierarchical navigation and URL/Path mapping for Rupa applications.

---

## 1. Core Concepts

- **Routes**: Declarative definitions of application paths and their associated components.
- **History Port**: An interface for managing navigation history (Memory, Browser, or Hash-based).
- **Reactive State**: The current path and parameters are exposed as Signals, ensuring the UI updates automatically upon navigation.

---

## 2. Technical Characteristics

- **Hierarchical**: Supports nested routes and layouts.
- **Type-Safe Params**: Extract variables from paths (e.g., `/user/:id`) with automatic type conversion.
- **Guard Support**: Hooks for protecting routes (e.g., Auth guards).
