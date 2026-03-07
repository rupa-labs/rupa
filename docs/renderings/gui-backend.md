# Module: GUI Backend (`crates/rupa-engine/src/renderer/gui/mod.rs`) 🛠️

The central hub for all hardware-accelerated graphical rendering. It organizes the specialized engines required for WGPU-based output across Desktop and Mobile platforms.

---

## 🏗️ Core Responsibilities

1.  **Module Aggregation:** Exposes sub-modules like `batcher`, `renderer`, `text_renderer`, and `texture` to the rest of the framework.
2.  **Public API Smoothing:** Provides clean re-exports of common types so that higher layers of `rupa-engine` and `rupa-mobile` can communicate efficiently with the GPU.

---

## 🗝️ Sub-Module Registry

| Module | Responsibility |
| :--- | :--- |
| **`renderer`** | Implementation of `trait Renderer` for the GPU. |
| **`batcher`** | Buffers vertex data to minimize draw calls. |
| **`text_renderer`** | High-performance text painting using Glyphon. |
| **`texture`** | GPU memory management for image assets. |

---

## 🔄 Integration

The GUI Backend is conditionally compiled via the `desktop` or `mobile` feature flags. When active, it provides the primary visual surface for the **Platform Orchestrator**.
