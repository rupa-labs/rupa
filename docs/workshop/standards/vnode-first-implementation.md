# Technical Standard: VNode-First Component Implementation 🌳

This document defines the mandatory implementation pattern for all UI components in the Rupa Framework. Moving forward, components MUST prioritize the **VNode Representation** over manual painting to ensure cross-platform compatibility and efficient reconciliation.

---

## 1. The Core Shift: render() vs paint()

In the legacy model, components often performed manual draw calls in a `paint()` method. In the **VNode-First** model:

*   **`render() -> VNode` (Mandatory)**: The component describes its visual tree using platform-agnostic VNodes. This is the source of truth.
*   **`paint()` (Legacy/Internal)**: This method is now secondary. In most cases, it will be handled automatically by the `Execution System` (Renderer) consuming the VNode tree.

---

## 2. Recommended Implementation Pattern

An Artisan Component (e.g., `Button`) should be implemented as a composition of **Primitives** (Div, Flex, Text).

### A. Component Structure
```rust
pub struct Button {
    id: String,
    label: Signal<String>,
    variant: Signal<Variant>,
    // ... other signals
}
```

### B. The VNode-First render() method
Instead of returning `VNode::Empty`, the component should build a virtual sub-tree:

```rust
impl Component for Button {
    fn render(&self) -> VNode {
        // Compose the UI using standard VNode primitives
        VNode::element("button")
            .with_style(self.compute_style())
            .with_child(VNode::text(self.label.get()))
    }
}
```

---

## 3. Benefits of this Pattern

1.  **Platform Agnosticism**: The same `Button` logic can be rendered as a WGPU rectangle on Desktop, a `<div>` on Web, or Unicode boxes in TUI.
2.  **State Synchronization**: Because `render()` is reactive, it only runs when its dependent `Signals` change.
3.  **Surgical Updates**: The reconciliation engine compares the produced VNode tree with the previous snapshot and issues minimal patches to the hardware.
4.  **Simplified Testing**: You can unit-test a `Button` by asserting that `render()` returns the correct `VElement` with the expected styles and children.

---

## 4. Refactoring Roadmap

To transition `rupa-ui` to VNode-First, each component will undergo these steps:

1.  **Identify Primitives**: Map manual `paint()` calls to their equivalent `VNode` primitives (e.g., `draw_rect` -> `VNode::element("div")`).
2.  **Move Styles**: Move style calculation from `layout()`/`paint()` into a reactive `compute_style()` helper used in `render()`.
3.  **Deprecate Manual Paint**: Set `render()` to return the full tree and ensure the `Execution System` is capable of rendering it.
