# Module: GUI Batcher (`crates/rupa-engine/src/renderer/gui/batcher.rs`) 🟦

The Batcher is the efficiency engine of the GPU pipeline in the Rupa Framework. It solves the performance bottleneck of "over-communication" between the CPU and GPU by grouping multiple VNode patches into single draw calls.

---

## 🧠 Internal Anatomy

### 1. Data Aggregation
The Batcher maintains dynamic buffers of **Vertices** and **Indices**. During the **Patch Phase**, the `GuiRenderer` sends geometry requests to the Batcher, which appends the required vertex data (4 vertices and 6 indices per rectangle).

### 2. VRAM Syncing
- **Mechanism:** Uses `device.create_buffer_init` to map RAM vectors into high-speed VRAM buffers.
- **Auto-Clearing:** The batch is automatically cleared after every `present()` call to prepare for the next frame's reconciliation.

---

## 🗝️ API Anatomy

### `struct Vertex`
The binary layout expected by the WGSL shader (defined via `bytemuck`):
- `position`: `[f32; 2]` (x, y coordinates).
- `color`: `[f32; 4]` (rgba values).
- `rect_size` / `radius`: Metadata used by the fragment shader to calculate SDF-based rounding.

---

## 🚀 Performance
By grouping agnostic VNodes into batched geometry, we reduce "Draw Calls" from thousands to just a handful per frame, maintaining high performance across Desktop and Mobile targets.
