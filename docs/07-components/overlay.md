# API Reference: Overlays 🎭

Floating components used for focused interactions or additional information.

---

## 🏗️ Modal
### `Modal::new(is_open: Signal<bool>) -> Self`
A blocking, centered window.

| Method | Argument | Description |
| :--- | :--- | :--- |
| `.header(Box<dyn Component>)` | Component | Attaches a top section. |
| `.child(Box<dyn Component>)`  | Component | Appends a child to the modal content. |
| `.footer(Box<dyn Component>)` | Component | Attaches a bottom section. |

---

## 🏗️ Tooltip
### `Tooltip::new(text: impl Into<String>) -> Self`
A small floating description.

---

## 🎨 Layout Constraints (Stylable)
Modals have a default width of `400.0` logical units, which can be overridden via:
- `.style(w(f32))`
- `.style(bg(Color))`
