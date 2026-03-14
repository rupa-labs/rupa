# Module: Scene Core (`crates/rupa-engine/src/scene/mod.rs`) 🌳

The Scene Core is the spatial brain of the Rupa Framework. It acts as the Single Source of Truth (SSOT) for the entire UI hierarchy and its geometric resolution.

---

## 🧠 Internal Anatomy

### 1. Spatial State Machine
- **Role:** Lifecycle manager.
- **Mechanism:** Uses the `SceneState` enum to strictly define whether the UI geometry is `Empty` or `Resolved`. This prevents interactions with a scene that hasn't been calculated yet.

### 2. Hit-Discovery Engine
- **Role:** Inverse Coordinate Mapper.
- **Responsibility:** Implements the recursive hit-testing algorithm. It translates a physical screen coordinate into a `HitResult` containing the component path.

---

## 🗝️ API Anatomy

- **`resolve(root, measurer, w, h)`**: Orchestrates the full tree traversal and Taffy calculation during the Patch Phase.
- **`find_target(cursor_pos)`**: Returns a semantic `HitDiscovery` enum identifying the target component.

---

## 🔄 Interaction Flow
- **Scene Core -> Layout Engine:** Delegates mathematical geometric calculations to Taffy.
- **Input Dispatcher -> Scene Core:** Queries for interaction targets during the capture phase of the event loop.
