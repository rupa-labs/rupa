# Button Group 🔘🔘

A layout container specifically designed to group and align multiple buttons.

---

## 🏗️ Purpose
`ButtonGroup` ensures that multiple buttons (e.g., "Save", "Cancel") are spaced correctly and share a common alignment strategy.

---

## 🚀 Example
```rust
ButtonGroup::new()
    .child(Button::new("Cancel").variant(Variant::Secondary))
    .child(Button::new("Save").variant(Variant::Primary))
```
