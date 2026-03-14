# Tooltip 💬

A small, non-blocking contextual hint.

---

## 🏗️ Purpose
Tooltips provide additional information about an element when the user hovers over it or focuses on it. They are ideal for clarifying icons or providing brief help text.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.child(comp)`** | Sets the trigger element for the tooltip. |

---

## 🚀 Example
```rust
Button::new("?")
    .child(Tooltip::new("Click for support"))
```
