# Interaction States 🖱️

Conditional styling modifiers that respond to user interactions.

---

## 🏗️ Usage
Interaction states allow you to define how a component's appearance should change when it is hovered, pressed, or focused.

### API
| Method | Description | Example |
| :--- | :--- | :--- |
| **`.on_hover_style(mod)`** | Applied when pointer enters. | `.on_hover_style(bg(Blue))` |
| **`.on_active_style(mod)`**| Applied while pressed. | `.on_active_style(scale(0.9))` |
| **`.on_focus_style(mod)`** | Applied when focused. | `.on_focus_style(outline(2.0))` |

---

## 🚀 Example
```rust
Button::new("Interactive")
    .on_hover_style(bg(Color::Zinc700))
    .on_active_style(rounded(Scale::Sm))
```
