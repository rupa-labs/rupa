# Module: Graphics API (`crates/rupa-canvas/src/api.rs`) 🖌️

This module defines the high-level drawing interface for the Canvas component.

---

## 🏗️ `struct CanvasContext`
A wrapper around WGPU render passes that provides a fluent drawing API.

- **Primitives**: `draw_circle`, `draw_path`, `draw_mesh`.
- **State**: Manages transformation matrices, stroke styles, and fill patterns.
- **Batching**: Automatically batches similar draw calls to minimize state changes.
