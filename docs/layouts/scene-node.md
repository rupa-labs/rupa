# Module: Scene Node (`node.rs`) 📍

The Scene Node is a platform-agnostic handle that represents a single element within the Geometric Scene. It implements the **Dependency Inversion** principle by wrapping the underlying layout engine's identifiers.

---

## 🏗️ Core Responsibilities

1.  **Identifier Abstraction:** Wraps Taffy's `NodeId` into a Rupa Framework-owned `SceneNode` struct.
2.  **Opaque Context:** Prevents high-level components from needing to import or understand the internal workings of the layout library.
3.  **Handle Safety:** Provides a centralized way to pass geometric references between the Scene (L3) and the Renderer (L2).

---

## 🗝️ Key API Elements

### `struct SceneNode`
- `raw()`: Safely exposes the internal ID for usage within the Geometric Scene Layer.
- `From<NodeId>`: Allows easy conversion during the layout calculation phase.

---

## 🔄 Interaction
- **L3 (Scene Node) -> L2 (Renderer):** Used to identify which geometric data to retrieve from the GPU or Terminal buffer.
- **L3 (Scene Node) -> L5 (Component):** Acts as the "body handle" stored within a component's View.
