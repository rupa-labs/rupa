# Renderer Port Architecture 🎨

The **Renderer Port** is the foundational contract for all visual manifestation backends in the Rupa Framework. It defines the "Suci" (Sacred) contract that bridges the Agnostic Kernel to platform-specific hardware drawing.

---

## 1. Core Interfaces

### A. Renderer Trait
The primary contract that all backends (WGPU, Crossterm, DOM) must implement.
- **`render_patch()`**: Executes a structural change (Add, Remove, Update) identified during reconciliation.
- **`draw_*()`**: Low-level primitives for drawing rectangles, text, and outlines.
- **`present()`**: Finalizes the frame and displays it on the target hardware.

### B. TextMeasurer Trait
A specialized interface used during the **Layout Phase**.
- **`measure()`**: Returns the physical dimensions of a string given a specific style and scale.

---

## 2. Technical Context

- **Stateless Execution**: The Renderer receives `Patch` instructions and executes them. It does not hold the application state.
- **Unit Agnostic**: While the trait uses floats for coordinates, the implementation determines if these are Pixels (GUI) or Cells (TUI).
- **Batching Support**: Designed to support batch rendering for high-performance graphics.

---

## 3. Implementation Workflow

1. **Reconciliation**: The Kernel identifies changes.
2. **Dispatch**: Patches are sent to the `Renderer`.
3. **Execution**: The `Renderer` translates patches into hardware instructions (e.g., Draw Calls or ANSI sequences).
4. **Presentation**: The hardware surface is updated.
