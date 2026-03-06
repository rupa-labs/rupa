# Module: Viewports (`viewport.rs`) 🔭

Viewports provide a window into an infinite 2D canvas. They are the primary tool for building zoomable, pannable interfaces like maps, design tools, or complex dashboards.

---

## 🧠 Internal Anatomy

### 1. The Camera Logic
- **State:** Tracks `offset` (Vec2) and `zoom` (f32) via reactive Signals.
- **Interactivity:** Translates `PointerScroll` into zoom changes and `PointerDrag` into offset shifts.

### 2. The Spacial View
- **Bridge:** Communicates with the **RenderCore (L2)** to set the global camera state before painting its children.
- **Infrastructure:** Composes `ViewCore` but also manages the **Inverse Coordinate Transformation** required for hit-testing children when zoomed.

---

## 🗝️ Public API

- `Viewport::new(content)`: Wraps a component tree in a zoomable canvas.
- `.pannable(bool)`: Enables/disables click-and-drag panning.
- `.zoomable(bool)`: Enables/disables scroll-based zooming.

---

## 🔄 Interaction Flow
- **L8 (Viewport) -> L2 (RenderCore):** Syncs camera matrices.
- **L8 (Viewport) -> L1 (Input):** Captures multi-touch and scroll signals.
