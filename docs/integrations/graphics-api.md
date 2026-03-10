# Graphics API (WGPU Infrastructure) 🎮

The hardware acceleration composite assembly of the Rupa Framework utilizes **WGPU**, a cross-platform, safe, and modern graphics API for Rust. It acts as the bridge between the framework and the physical GPU, abstracting away the differences between Vulkan, Metal, DX12, and WebGPU.

---

## 🏗️ The GPU Lifecycle

When a Rupa Framework application starts, it performs a sequence of hardware handshakes:

1.  **Instance Creation:** The entry point for WGPU. it discovers the available graphics backends on your system.
2.  **Adapter Selection:** Rupa Framework requests an "Adapter" (a handle to your physical GPU). By default, we prioritize **High Performance** for smooth UI rendering.
3.  **Device & Queue:** 
    *   The **Device** is the logical connection to the GPU used to create resources (buffers, textures).
    *   The **Queue** is the command lane where draw instructions are submitted.
4.  **Surface & Configuration:** The connection between the GPU and the OS window. Rupa Framework automatically configures the **Swapchain** (the sequence of images shown on screen) to match the window's size and pixel format.

---

## 🎨 Pixel Formats & SRGB

Rupa Framework enforces **SRGB** color spaces at the hardware level. This ensures that the procedural colors calculated by the styling system (OKLCH) are presented with maximum color accuracy and no "gamma-shifting" across different monitors.

## ⚡ Hardware Acceleration

Every visual element in Rupa Framework is hardware-accelerated. Instead of using the CPU to calculate pixels, Rupa Framework:
- Uploads vertex data to **GPU Buffers**.
- Uses **Shaders (WGSL)** to calculate anti-aliased corners and shadows in parallel across thousands of GPU cores.
- Manages **Texture Atlases** to keep icons and text glyphs in high-speed VRAM.

---

## 🛠️ Internal Module Reference
The core logic for this layer resides in:
- `crates/rupa-engine/src/renderer/gui/renderer.rs`: Device initialization and frame management.
- `crates/rupa-engine/src/renderer/gui/texture.rs`: GPU memory management for images.
