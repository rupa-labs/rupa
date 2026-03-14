# Sizing 📐

Defines the explicit or relative dimensions of a component.

---

## 🏗️ Usage
Use sizing utilities to control how much space a component occupies on the screen.

### API
| Method | Description | Example |
| :--- | :--- | :--- |
| **`.w(val)`** | Sets an explicit width. | `.w(200.0)` |
| **`.h(val)`** | Sets an explicit height. | `.h(Scale::Xl)` |
| **`.w_full()`** | Sets width to 100% of parent. | `.w_full()` |
| **`.h_full()`** | Sets height to 100% of parent. | `.h_full()` |
| **`.size(w, h)`**| Sets both dimensions at once. | `.size(100.0, 50.0)` |

---

## 🚀 Example
```rust
Div::new()
    .w_full()
    .h(Scale::Xl6)
    .bg(Color::Blue)
```
