# Border Radius 🟠

Smooth out corners with standardized rounding.

---

## 🏗️ Usage
Controls the curvature of a component's corners using the Artisan scaling system.

### API
| Method | Description | Example |
| :--- | :--- | :--- |
| **`.rounded(val)`** | Sets corner radius. | `.rounded(Scale::Md)` |
| **`.rounded_full()`**| Makes the component a pill or circle. | `.rounded_full()` |

---

## 🚀 Example
```rust
Button::new("Rounded Action")
    .rounded(Scale::Lg)
```
