# Pulse Animation 💓

A continuous scaling or opacity effect used to draw attention to an element.

---

## 🏗️ Purpose
`Pulse` is often used for notification badges, active recording indicators, or "breath" effects on hero elements.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.pulse()`** | Applies a standard 1.05x scale pulse. |
| **`.pulse_opacity()`**| Animates opacity between 0.5 and 1.0. |

---

## 🚀 Example
```rust
Badge::new("LIVE")
    .variant(Variant::Danger)
    .pulse()
```
