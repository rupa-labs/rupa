# Module: Renderer Interface (`mod.rs`) 🎼

This module defines the **Agnostic Bridge** for all visual output. It provides the universal contract that allows the framework to speak a single language to various hardware backends.

---

## 🧠 Internal Anatomy

### 1. RenderCore (The Spacial Heart)
- **Role:** Shared State Container.
- **Responsibility:** Stores the "Visual Perspective" of the application, including the camera zoom level, camera offset (panning), and the logical canvas size. It is composed into specific backend renderers.

### 2. The Universal Contract (`trait Renderer`)
- **Role:** Architectural Boundary.
- **Responsibility:** Defines the primitive drawing methods (`draw_rect`, `draw_text`) and clipping management. By requiring every backend to implement this trait, Rupaui ensures that components remain platform-independent.

---

## 🗝️ API Anatomy

### `trait Renderer`
- `core()` / `core_mut()`: Access to the shared spacial state.
- `draw_rect()`: Unified method for geometry.
- `draw_text()`: Unified method for typography.
- `present()`: The "Flush" command that sends data to the hardware.

---

## 🔄 Interaction Flow
- **L2 (Orchestrator) -> L2 (Backend):** Delegates commands based on feature flags.
- **L5 (Component) -> L2 (Renderer):** Components call these methods during their `paint()` phase.
