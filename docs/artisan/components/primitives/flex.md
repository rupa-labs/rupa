# Flex 🧬

The raw foundation for all directional layout components in Rupa.

---

## 🏗️ Purpose
`Flex` provides direct access to the Flexbox layout engine. Use this when `VStack` or `HStack` defaults are not enough for your specific alignment needs.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.flex_row()`** | Sets direction to horizontal. |
| **`.flex_col()`** | Sets direction to vertical. |
| **`.items_center()`**| Aligns items on the cross axis. |

---

## 🚀 Example
```rust
Flex::new()
    .flex_row()
    .justify_between()
    .child(Sidebar::new())
    .child(MainContent::new())
```
