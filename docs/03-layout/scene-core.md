# Module: Scene Core (`mod.rs`) 🌳

The Scene Core is the spatial brain of Rupaui. it acts as the Single Source of Truth (SSOT) for the entire UI hierarchy and its geometric resolution.

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

- `resolve(root, measurer, w, h)`: orchestrates the full tree traversal and Taffy calculation.
- `find_target(cursor_pos)`: returns a semantic `HitDiscovery` enum.

---

## 🔄 Interaction Flow
- **L3 (Scene Core) -> L3 (Layout Engine):** Delegates geometric calculations.
- **L1 (Dispatcher) -> L3 (Scene Core):** Queries for interaction targets.
