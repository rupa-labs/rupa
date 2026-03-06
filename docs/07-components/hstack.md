# Component: HStack ↔️

The `HStack` (Horizontal Stack) is a specialized layout component that arranges its children in a horizontal row. it is built on top of the `Flex` primitive with a pre-configured horizontal direction.

---

## 🧠 Internal Anatomy

### 1. HStack Logic
- **Role:** Horizontal collection management.
- **Responsibility:** Manages the row-based order of components. It provides a clean API for horizontal gap management.

### 2. HStack View
- **Infrastructure:** Composes **`ViewCore`**.
- **Mapping:** Automatically sets `FlexDirection::Row` in the Taffy tree. This ensures that children are aligned side-by-side during the geometric resolution phase.

---

## 🗝️ Public API (Usage)

### Constructor
- `HStack::new()`: Initializes an empty horizontal stack.

### Methods
- `.gap(f32)`: Sets the horizontal spacing between items.
- `.child(Box<dyn Component>)`: Appends an element to the row.

---

## 💻 Usage Example

```rust
HStack::new()
    .gap(16.0)
    .child(Box::new(Button::new("Cancel")))
    .child(Box::new(Button::new("Confirm")))
```
