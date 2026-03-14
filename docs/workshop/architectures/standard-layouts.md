# Standard Layouts Architecture 📐

**Standard Layouts** provide universal Flexbox and CSS Grid patterns built upon the **Layout Kernel**.

---

## 1. Primary Patterns

- **Flex Layouts**: `VStack`, `HStack`, `Flex`.
- **Grid Layouts**: `Grid`, `Col`, `Row`.
- **Containers**: `Section`, `Divider`, `Viewport`.

---

## 2. Technical Identity

These are high-level **Agnostic UI** components that abstract away the complexity of raw Taffy style declarations into a semantic, fluent API.
```rust
VStack::new().items_center().gap(Step::S2)
```
