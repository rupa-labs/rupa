# Module: Renderer Interface (`crates/rupa-core/src/renderer.rs`) 🎼

This module defines the **Agnostic Bridge** for all visual output in the Rupa Framework. It provides the universal contract that allows the core engine to speak a single language to various hardware backends (WGPU, TUI, etc.).

---

## 🧠 Core Architecture

### 1. RenderCore (The Spatial Heart)
- **Role:** Shared State Container.
- **Responsibility:** Stores the "Visual Perspective" of the application, including the camera zoom level, camera offset (panning), and the logical canvas size. This core is shared across all backend renderers.

### 2. The Universal Contract (`trait Renderer`)
- **Role:** Architectural Boundary.
- **Responsibility:** Defines the primitive drawing methods and lifecycle commands. By requiring every backend to implement this trait, the Rupa Framework ensures that the higher layers (VNode reconciler) remain platform-independent.

---

## 🗝️ API Anatomy

### `trait Renderer`
- **`render_patch(patch)`**: The primary entry point during the **Patch Phase**. It consumes structural changes identified during VNode diffing and translates them into backend commands.
- **`present()`**: The final "Flush" command that submits command buffers to the GPU or writes the character buffer to the terminal.
- **`resize(width, height)`**: Synchronizes the renderer's internal viewport with the physical surface dimensions.

---

## 🔄 Interaction Flow

1.  **VNode Reconciliation**: `rupa-core` identifies a difference between the old and new VNode trees.
2.  **Patch Generation**: A set of instructions (Create, Update, Delete, Reorder) is generated.
3.  **Renderer Execution**: The active renderer (e.g., `GuiRenderer`) consumes these patches:
    *   **Create**: Allocates new layout nodes and buffers vertex data.
    *   **Update**: Modifies existing styles or text content.
    *   **Delete**: Reclaims GPU memory and layout resources.
4.  **Hardware Submission**: The `present()` method is called to finalize the frame.
