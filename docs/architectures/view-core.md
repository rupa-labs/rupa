# Module: View Core (`crates/rupa-core/src/view.rs`) 🦴

> **⚠️ Architectural Note:** In the modern **VNode Architecture**, `ViewCore` has evolved from a component-level mixin into an internal mechanism for managing the lifecycle of rendered outputs and their physical scene nodes.

---

## 🧠 Core Responsibilities

The View Core provides the infrastructure for translating agnostic `VNodes` into physical `SceneNodes` within the rendering engine.

### 1. Snapshot Management
- **VNode Tracking**: Stores the "Previous VNode" to enable efficient diffing when a component re-renders.
- **Scene Handles**: Maintains the mapping to the physical `SceneNode` and Taffy layout node.

### 2. Reconciliation Logic
- **Diff & Patch**: Orchestrates the process of comparing two VNode trees and issuing surgical updates to the renderer.
- **Hydration Support**: Provides the hooks needed to "re-attach" reactive signals to an existing UI tree (essential for SSR to Client-side transition).

---

## 🗝️ API & Lifecycle

- **`reconcile(old, new)`**: The primary engine for identifying changes between two virtual snapshots.
- **`mount(vnode)`**: The initial entry point that creates the physical representation for the first time.
- **`unmount()`**: Ensures that all GPU resources and layout nodes are safely released.

---

## 🛡️ Structural Integrity

By centralizing reconciliation in `ViewCore`, the Rupa Framework ensures:
1.  **Consistency**: Every component follows the same diffing rules.
2.  **Performance**: Minimal mutations are sent to the hardware-accelerated layers.
3.  **Safety**: All tree modifications are performed in a single, well-defined reactive phase, preventing race conditions or inconsistent UI states.
