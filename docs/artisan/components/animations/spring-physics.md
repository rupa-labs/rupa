# Spring Physics 💫

Physics-based motion that provides realistic, interruptible animations.

---

## 🏗️ Usage
Springs use physical constants instead of fixed durations, allowing for more natural and responsive UI behavior.

### API Methods
| Method | Description |
| :--- | :--- |
| **`.spring()`** | Applies default spring physics. |
| **`.spring_custom(mass, stiffness, damping)`** | Fine-tune the physical response. |

---

## 🚀 Example
```rust
let pos = Signal::new(0.0);

Div::new()
    .translate_x(pos.get())
    .spring() // The movement will feel "elastic"
```
