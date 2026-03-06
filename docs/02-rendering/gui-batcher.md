# Module: GUI Batcher (`gui/batcher.rs`) 🟦

The Batcher is the efficiency engine of the GPU pipeline. It solves the performance bottleneck of "over-communication" between the CPU and GPU.

---

## 🧠 Internal Anatomy

### 1. Data Aggregation
The Batcher maintains growing vectors of **Vertices** and **Indices**. Every time `draw_rect` is called in the renderer, the Batcher appends 4 vertices and 6 indices to these collections.

### 2. VRAM Syncing
- **Mechanism:** Uses `device.create_buffer_init` to map RAM vectors directly into high-speed VRAM.
- **Auto-Clearing:** The batch is automatically cleared after every `flush()` call to prepare for the next frame.

---

## 🗝️ API Anatomy

### `struct Vertex`
The binary layout expected by the WGSL shader:
- `position`: x, y coordinates.
- `color`: rgba values.
- `rect_size` / `radius`: Metadata used by the fragment shader to calculate SDF rounding.

---

## 🚀 Performance
By grouping geometry, we reduce "Draw Calls" from thousands to just a handful per frame, maintaining a steady 60+ FPS even with complex UIs.
