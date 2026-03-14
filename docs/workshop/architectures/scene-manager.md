# Scene Manager Architecture 🗺️

The **Scene Manager** is the Tier 2 system responsible for managing the physical spatial representation of the UI and performing hit-testing.

---

## 1. Core Responsibilities

- **Geometric Tracking**: Maintains the link between `VNodes` and their physical `SceneNodes` (Handles from the Layout Kernel).
- **Hit-Testing**: Identifies which UI element is located at a specific **Pointer** coordinate.
- **Layer Management**: Handles the **Global Z-Stack**, ensuring that overlays and modals are prioritized during interaction and rendering.
- **Isolation Enforcement**: Implements **Focus Traps** and input blocking for modal components.

---

## 2. Technical Context

- **Coordinate Resolution**: Translates global screen coordinates into local component coordinates.
- **Interaction Priority**: Manages the multi-layer system (Primary Content vs. Global Overlay Layer).
- **Cursor Resolution**: Identifies the topmost hovered component to determine the required OS cursor icon.

---

## 3. Relationship with Other Systems

- **Input Dispatcher**: Uses the Scene Manager to find the target of a `Pointer` event.
- **Layout Kernel**: Provides the physical dimensions used for hit-testing calculations.
- **Renderer Port**: Consumes the Z-order information to ensure correct draw call sequencing.
