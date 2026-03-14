# Spatial Awareness & Hit-Testing 🎯

Spatial Awareness is the ability of Rupa Framework to bridge the gap between a 2D screen coordinate (where the user clicked) and the complex hierarchy of the Scene Graph.

---

## 🏗️ Core Responsibilities

1.  **Hit-Testing:** Mapping physical input coordinates (from the OS) to the correct component within the VNode tree.
2.  **Semantic Discovery:** Communicating the search results using the `HitDiscovery` enum (`Missed` or `Found`).
3.  **Z-Index Resolution:** Ensuring that the component "on top" visually is the one that receives the input signal.

---

## 🔄 The Interaction Bridge

When an input signal arrives from the Platform Runner:
1.  **Capture:** `PlatformCore` receives a coordinate.
2.  **Request:** It asks `SceneCore` to `find_target`.
3.  **Discovery:** `SceneCore` traverses the tree and returns a `HitDiscovery::Found(path)`.
4.  **Dispatch:** The `InputDispatcher` executes the logic path stored within the discovery result.

---

## 🛠️ Internal Module Reference
- `crates/rupa-engine/src/scene/mod.rs`: The provider of the spatial search logic.
- `crates/rupa-engine/src/platform/dispatcher.rs`: The consumer of the discovery result.
