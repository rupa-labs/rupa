# API Reference: Feedback & Status 📢

This module contains non-interactive or status-reporting components used to inform the user about application state.

---

## 🏗️ Alert
### `Alert::new(message: impl Into<String>) -> Self`
A highlighted box for notifications.
- **Default Variant:** `Variant::Danger`

| Method | Argument | Description |
| :--- | :--- | :--- |
| `.variant(v)` | `Variant` | Sets the alert level (Success, Warning, Danger). |

---

## 🏗️ Progress
### `Progress::new(value: Signal<f32>) -> Self`
A horizontal progress bar.
- **Range:** Expects a value between `0.0` and `1.0`.

---

## 🏗️ Badge
### `Badge::new(label: impl Into<String>) -> Self`
A small pill-shaped label.

---

## 🏗️ Spinner & Skeleton
### `Spinner::new() -> Self`
An animated loading circle.

### `Skeleton::new() -> Self`
A placeholder box used during content loading.

---

## 🎨 Styling (Stylable)
Feedback components heavily rely on semantic colors:
- `.style(bg(Color::Semantic("surface-variant", None)))`
- `.style(rounded_full())` (Common for badges)
