# Module: Route Configuration (`crates/rupa-router/src/route.rs`) 📍

This module defines the individual route definitions and their metadata.

---

## 🧩 `struct Route`
A declarative object mapping a URL path to a component.

- **Dynamic Parameters**: Supports paths like `/user/:id` with reactive parameter extraction.
- **Lazy Loading**: Integrated with `VComponent` metadata to support code-splitting.
