# VNode Architecture 🌳

The **VNode (Virtual Node)** is the universal language of the Rupa Framework. it acts as a platform-agnostic intermediary between high-level UI components and low-level rendering engines.

---

## 1. The Core Purpose

In a multi-platform framework, components should not be tied to a specific rendering technology (like WGPU for pixels or `web-sys` for the DOM). Instead, components describe **what** the UI should look like by producing a tree of VNodes.

The rendering engines (`rupa-engine`, `rupa-client`, `rupa-server`) then consume this tree and translate it into platform-specific instructions.

### Benefits:
*   **Headless Testing:** Components can be tested by asserting the produced VNode tree without needing a window or GPU.
*   **Multi-Targeting:** The same component can be rendered as a GPU primitive, an ANSI character, or an HTML tag.
*   **Performance:** Enables efficient "Diffing" and "Patching," ensuring only changed parts of the UI are updated.

---

## 2. Anatomy of a VNode

A VNode is a lightweight, serializable structure that captures the intent of a UI element.

```rust
pub enum VNode {
    /// A structural element (e.g., Div, VStack, Button).
    Element(VElement),
    /// Plain text content.
    Text(String),
    /// A placeholder for a component that hasn't been resolved yet.
    Component(VComponent),
    /// An empty node (used for conditional rendering).
    Empty,
}

pub struct VElement {
    pub tag: String,
    pub style: Style,
    pub attributes: Attributes,
    pub children: Vec<VNode>,
    pub key: Option<String>, // Used for efficient list reconciliation
}
```

---

## 3. The Lifecycle: Build, Diff, Patch

The Rupa Framework follows a reactive lifecycle driven by the VNode tree.

1.  **Build:** When a `Signal` changes, the affected component's `render()` method is called, producing a new VNode sub-tree.
2.  **Diff:** The framework compares the new sub-tree with the previous one to identify minimal changes.
3.  **Patch:** The identified changes are sent to the active renderer:
    *   **Native Engine:** Updates Taffy layout nodes and issues new GPU draw calls.
    *   **Web Client:** Performs surgical DOM manipulations (e.g., `setAttribute`, `textContent`).
    *   **Server:** Serializes the tree into an initial HTML string.

---

## 4. Implementation Strategy

To maintain the **Atoms & Composites** standard, the VNode implementation is split:

*   **`rupa-vnode` (Atom):** Contains the core `VNode` enum, `VElement` structs, **and the foundational Style data models** (Color, Spacing, Layout). It is 100% agnostic and serializable.
*   **`rupa-core` (Composite):** Integrates VNodes into the `Component` trait and provides the reconciliation (Diff/Patch) engine.

---

## 5. Architectural Mandate

*   **Serialization:** Every VNode must be serializable to support the **Polyglot Hybrid** vision (JS/TS interoperability).
*   **Immutability:** VNodes are treated as immutable snapshots. Once built, they are not modified; instead, a new tree is produced.
*   **Efficiency:** Use `Box` or `Arc` for children to minimize memory overhead during tree traversal.
