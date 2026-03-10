# VNode Reconciliation Engine 🔄

The **Reconciliation Engine** is the heart of the Rupa Framework's performance and platform-agnosticism. It is responsible for identifying the minimal set of changes between two Virtual Tree snapshots and translating them into atomic instructions (**Patches**) for the active renderer.

---

## 1. The Build-Diff-Patch Lifecycle

The framework follows a non-destructive update cycle:

1.  **Build Phase**: A reactive change triggers a component's `render()` method, producing a new `VNode` tree.
2.  **Diff Phase**: The Core Engine compares the *New Tree* with the *Previous Tree* (stored in the component's `ViewCore`).
3.  **Patch Phase**: The generated `PatchSet` is sent to the `Renderer` (GPU, TUI, or Web) for surgical execution.

---

## 2. The Patch Instruction Set

A `Patch` is an atomic instruction that tells the renderer exactly what to change. This avoids full-tree re-layouts and minimizes expensive I/O operations (like GPU draw calls or DOM updates).

```rust
pub enum Patch {
    /// Create a new element and append it to a parent.
    Create {
        parent_id: Option<String>,
        node: VNode,
        index: usize,
    },
    /// Update attributes or styles of an existing node.
    Update {
        id: String,
        changes: Vec<UpdateType>,
    },
    /// Move an existing node to a new index within the same or different parent.
    Move {
        id: String,
        from_index: usize,
        to_index: usize,
    },
    /// Remove a node from the tree.
    Delete {
        id: String,
    },
    /// Replace an entire sub-tree (used when node tags differ).
    Replace {
        id: String,
        new_node: VNode,
    }
}

pub enum UpdateType {
    Style(Style),
    Attribute(String, String),
    Text(String),
}
```

---

## 3. The Diffing Algorithm (Heuristics)

Comparing two arbitrary trees is an $O(n^3)$ problem. Rupa uses a **heuristic algorithm** with $O(n)$ complexity based on these assumptions:

### A. Tag Stability
If two elements have different tags (e.g., a `Div` changed to a `Button`), the engine assumes the entire sub-tree is different and uses a `Replace` patch instead of diffing children.

### B. Keyed Reconciliation
For lists of children, Rupa uses an optional `key` property. 
- **With Keys**: The engine tracks nodes even if they are reordered, inserted, or removed. This enables surgical `Move` patches instead of expensive `Delete/Create` cycles, preserving the internal state and performance of the elements.
- **Without Keys**: The engine performs a simple index-based comparison. This is efficient for static lists but may cause re-renders if elements change positions.

### C. Deep Diffing Optimization
To minimize updates to the active renderer, the engine performs fine-grained comparisons:
- **Style Diffing**: Instead of replacing entire style objects, Rupa identifies changed sub-properties (e.g., just the padding or background) and sends atomic updates.
- **Attribute Diffing**: Identifies exactly which attributes were added, removed, or changed.

### D. Component Memoization
If a `VComponent`'s props haven't changed (checked via partial equality), the engine skips diffing its children entirely, assuming the output remains the same.

---

## 4. Implementation Strategy in `rupa-core`

The reconciliation engine will be implemented in `crates/rupa-core/src/reconciler/`:

1.  **`mod.rs`**: High-level `reconcile(old, new)` entry point.
2.  **`diff.rs`**: Core logic for generating `Patch` sets.
3.  **`patch.rs`**: Data structures for the instruction set.

---

## 5. Architectural Benefits

*   **Platform Agnostic**: The `reconcile` function doesn't know about WGPU or HTML. It only speaks `VNode` and `Patch`.
*   **Performance**: Minimizes layout recalculations in Taffy and paint calls in WGPU.
*   **Testability**: The engine can be unit-tested by asserting `reconcile(tree_a, tree_b) == expected_patches` without any UI environment.
