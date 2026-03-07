# Primitive: Overlay 🌫️

The `Overlay` primitive is specialized for non-flow positioning. It is the structural basis for elements that need to break free from the normal Flex/Block layout, such as dropdowns, tooltips, or floating action buttons.

---

## 🧠 Internal Anatomy

### 1. OverlayLogic
- **Role:** Floating child management.
- **Responsibility:** Manages elements that exist visually "on top" of other layers.

### 2. OverlayView
- **Infrastructure:** Composes `ViewCore`.
- **Taffy Mapping:** Sets the Taffy `Position` property to `Absolute`. This informs the layout engine to ignore this node when calculating the flow of sibling elements.

---

## 🗝️ Public API (Usage)

### Constructor
- `Overlay::new()`: Initializes an absolute positioning container.

### Methods
- `.child(Box<dyn Component>)`: Appends a child to the overlay layer.

---

## 🎨 Layout Constraints
Users typically combine `Overlay` with specific coordinate modifiers:
```rust
Overlay::new()
    .style((
        bg(Color::Rgba(0.0, 0.0, 0.0, 0.5)), // Backdrop
        w_full(),
        h_full()
    ))
```

---

## ⚡ Spatial Awareness
Because it uses `Position::Absolute`, the `InputDispatcher` treats `Overlay` hits with high priority, ensuring that floating elements capture interactions before the background content.
