# Element: Grid 🏁

The `Grid` component is a powerful 2D layout engine based on CSS Grid. It allows you to align elements into columns and rows with precise control over spanning and gaps.

---

## 🏗️ Architecture

- **Logic:** Uses Taffy's Grid implementation.
- **Mapping:** Renders as a `div` tag with `Display::Grid`.
- **System:** Supports fractional units (`fr`), absolute pixels, and auto-placement.

---

## 🗝️ API & Usage

### Grid Container
```rust
Grid::new()
    .cols(3)      // Create 3 equal columns
    .grid_gap(1.0) // Set gap between items
    .child(...)
```

### Grid Items (Children)
You can use grid modifiers on any child of a Grid:
```rust
Text::new("Span Me")
    .col_span(2) // Occupy 2 columns
    .row_span(2) // Occupy 2 rows
```

### Methods
- `.cols(n)`: Shorthand for `n` equal columns (`1fr`).
- `.rows(n)`: Shorthand for `n` equal rows (`1fr`).
- `.grid_gap(val)`: Sets both column and row gaps.
- `.span_full()`: Makes the item occupy all available columns.

---

## 🌳 VNode Representation

When `render()` is called, a `Grid` produces:
```rust
VNode::Element(VElement {
    tag: "div",
    style: /* Style with Display::Grid */,
    children: /* rendered children */,
    key: Some(id),
})
```
