# Primitive: Flex ↔️↕️

The `Flex` primitive provides a one-dimensional layout system. It is the architectural vehicle for arranging children in horizontal or vertical stacks with precise distribution rules.

---

## 🧠 Internal Anatomy

### 1. FlexLogic
- **Role:** Directional container logic.
- **Responsibility:** Manages the ordered list of children and provides the context for whether items should flow as a row or a column.

### 2. FlexView
- **Infrastructure:** Uses `ViewCore` for geometric state.
- **Taffy Mapping:** Explicitly sets the Taffy `Display` property to `Flex`. It translates high-level modifiers like `justify_center()` or `gap(10.0)` into Flexbox algorithm parameters.

---

## 🗝️ Public API (Usage)

### Constructor
- `Flex::new()`: Initializes a new Flex container (Default: Row).

### Chaining Methods
- `.row()`: Sets flex-direction to Row.
- `.col()`: Sets flex-direction to Column.
- `.child(Box<dyn Component>)`: Adds a child to the stack.

---

## 🎨 Visual Identity
`Flex` is often used to build complex components like `Navbar` or `ButtonGroup`.
```rust
Flex::new()
    .col()
    .style(gap(8.0))
    .child(Box::new(Text::new("A")))
    .child(Box::new(Text::new("B")))
```

---

## ⚡ Interaction
- **Coordinate Space:** `FlexView` ensures that child global positions are calculated correctly along the main and cross axes during the paint phase.
