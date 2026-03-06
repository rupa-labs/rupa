# Component: VStack ↕️

The `VStack` (Vertical Stack) is a specialized layout component that arranges its children in a vertical column. it is built on top of the `Flex` primitive with a pre-configured vertical direction.

---

## 🧠 Internal Anatomy

### 1. VStack Logic
- **Role:** Vertical collection management.
- **Responsibility:** Delegates child storage to the underlying `Flex` logic. It provides a semantic interface for setting gaps between vertical items.

### 2. VStack View
- **Infrastructure:** Composes **`ViewCore`** via the inner Flex primitive.
- **Mapping:** Automatically sets `FlexDirection::Col` in the Taffy tree. It ensures that vertical spacing is calculated accurately during the layout phase.

---

## 🗝️ Public API (Usage)

### Constructor
- `VStack::new()`: Initializes an empty vertical stack.

### Methods
- `.gap(f32)`: Sets the vertical spacing between children.
- `.child(Box<dyn Component>)`: Appends an element to the column.

---

## 💻 Usage Example

```rust
VStack::new()
    .gap(10.0)
    .child(Box::new(Text::new("Header")))
    .child(Box::new(Text::new("Content body...")))
```
