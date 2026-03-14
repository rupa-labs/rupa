# Badge 🏷️

Small visual indicators for counts, labels, or statuses.

---

## 🏗️ Purpose
`Badge` is perfect for displaying a notification count on an icon or a status label (e.g., "New", "Stable") next to text.

---

## 🚀 Example
```rust
HStack::new()
    .child(Text::new("Messages"))
    .child(Badge::new("5").variant(Variant::Primary))
```
