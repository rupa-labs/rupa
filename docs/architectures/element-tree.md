# Module: Element Tree Utilities (`element_tree.rs`) 🌳

The Element Tree module provides high-level recursive utilities for navigating and inspecting the component hierarchy. It serves as an essential toolset for both internal framework logic and debugging.

---

## 🏗️ Core Responsibilities

1.  **Node Discovery:** Allows finding specific components within a deep tree using their unique IDs.
2.  **Tree Walking:** Implements recursive traversal (`walk`) to execute logic across every active component.
3.  **Metrics:** Provides tree-wide statistics, such as the total number of active components.
4.  **Diagnostics:** Generates human-readable debug dumps of the UI structure.

---

## 🗝️ Key API Elements

### `struct ElementTree`
- `find(root, id)`: Recursively searches for a component handle.
- `walk(root, closure)`: Performs a depth-first traversal of the tree.
- `count(root)`: Returns the total component count.
- `debug_dump(root)`: Prints the hierarchy to the console/log.

---

## 🔄 Interaction
- **Element Tree -> Component:** Operates on the `Component` trait objects.
- **Element Tree -> Accessibility:** Can be used to generate the simplified accessibility tree required by Platform Runners.
