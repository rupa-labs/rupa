# Padding 📏

Controls the inner space of a component.

---

## 🏗️ Usage
Padding pushes content away from the component's boundaries. It supports both semantic `Scale` and absolute `f32` values.

### Chaining API
| Method | Description | Example |
| :--- | :--- | :--- |
| **`.p(val)`** | Sets padding on all sides. | `.p(Scale::Md)` |
| **`.px(val)`** | Sets horizontal padding. | `.px(16.0)` |
| **`.py(val)`** | Sets vertical padding. | `.py(Scale::Sm)` |
| **`.pt(val)`** | Sets top padding. | `.pt(Scale::Xs)` |
| **`.pr(val)`** | Sets right padding. | `.pr(4.0)` |
| **`.pb(val)`** | Sets bottom padding. | `.pb(Scale::Lg)` |
| **`.pl(val)`** | Sets left padding. | `.pl(8.0)` |

---

## 🚀 Example
```rust
VStack::new()
    .p(Scale::Lg)
    .child(Text::new("Spacious Content"))
```
