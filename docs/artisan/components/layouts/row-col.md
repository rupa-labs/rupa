# Row & Col 📐

Lightweight, semantic aliases for flex containers.

---

## 🏗️ Purpose
- **`Row`**: Alias for `HStack` with no-wrap defaults.
- **`Col`**: Alias for `VStack` with stretch defaults.

Use these for rapid, readable layout assembly when you don't need the full `Flex` API.

---

## 🚀 Example
```rust
Row::new().child(Item1).child(Item2)
Col::new().child(Header).child(Content)
```
