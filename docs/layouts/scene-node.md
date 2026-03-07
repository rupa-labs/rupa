# Module: Scene Node (`crates/rupa-engine/src/scene/node.rs`) 📍

The Scene Node is a platform-agnostic handle that represents a single element within the Geometric Scene. It implements the **Dependency Inversion** principle by wrapping the underlying layout engine's identifiers.

---

## 🏗️ Core Responsibilities

1.  **Identifier Abstraction:** Wraps Taffy's `NodeId` into a Rupa Framework-owned `SceneNode` struct.
2.  **Opaque Context:** Prevents high-level components from needing to import or understand the internal workings of the layout library.
3.  **Handle Safety:** Provides a centralized way to pass geometric references between the Scene Graph and the active Renderer.

---

## 🗝️ Key API Elements

### `struct SceneNode`
- **`raw()`**: Safely exposes the internal Taffy ID for usage within the layout engine.
- **`From<NodeId>`**: Allows easy conversion during the layout calculation phase.

---

## 🔄 Interaction
- **Scene Node -> Renderer:** Used to identify which geometric data to retrieve from the GPU or Terminal buffer.
- **Scene Node -> ViewCore:** Acts as the internal "body handle" stored within a VNode's tracked state during reconciliation.
