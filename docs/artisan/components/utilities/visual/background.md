# Background ✨

Enhance components with surface colors and patterns.

---

## 🏗️ Usage
Background utilities define the visual foundation of a component's surface.

### API
| Method | Description | Example |
| :--- | :--- | :--- |
| **`.bg(color)`** | Sets an explicit color. | `.bg(Color::Zinc900)` |
| **`.bg_primary()`**| Uses the theme's primary color. | `.bg_primary()` |
| **`.bg_surface()`**| Uses the theme's surface color. | `.bg_surface()` |

---

## 🚀 Example
```rust
VStack::new()
    .bg_primary()
    .child(Text::new("Hero Banner"))
```
