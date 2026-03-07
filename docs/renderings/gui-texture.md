# Module: GUI Texture (`crates/rupa-engine/src/renderer/gui/texture.rs`) 🖼️

The Texture module manages raw pixel data within VRAM in the Rupa Framework. It acts as the GPU-side storage for images and icons referenced by VNodes.

---

## 🏗️ Core Responsibilities

1.  **GPU Memory Allocation:** Creates `wgpu::Texture` objects with optimal usage flags (Binding & Copying).
2.  **Asset Uploading:** Writes raw byte buffers into GPU memory using the `Queue` during the **Patch Phase**.
3.  **Sampler Management:** Configures how the GPU interpolates pixels (Nearest vs. Linear).
4.  **Bind Group Creation:** Automates the creation of WGPU bind groups required for shaders to access texture data.

---

## 🗝️ API Elements

### `struct Texture`
- **`from_bytes(device, queue, bytes, label)`**: Creates a fully ready GPU texture from an in-memory buffer.
- **`view`**: The logical view into the texture data used by the pipeline.
- **`bind_group`**: The handle passed to the render pipeline during the drawing of image-based VNodes.

---

## 🔄 Interaction

The `GuiRenderer` uses the Texture module to resolve `VNode::Image` or `VNode::Icon` references into physical GPU resources during the Patch Phase.
