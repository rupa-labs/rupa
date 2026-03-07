# Components: Form Controls 📝

Form controls in Rupa Framework are designed to be **Reactive by Default**. They act as a two-way bridge between user input signals and your application's state.

---

## 🧠 Internal Anatomy

Every form element (Input, Select, Checkbox, etc.) follows a consistent internal structure:

### 1. The Reactive Logic
- **Data Binding:** Holds a `Signal<T>` (e.g., `Signal<String>` for Input, `Signal<bool>` for Checkbox).
- **Validation:** (Planned) Internal logic to check data integrity before committing changes to the signal.
- **Interactivity:** Handles the translation of raw keystrokes or clicks into data mutations.

### 2. The Semantic View
- **State Visuals:** Automatically updates the component's appearance based on the bound signal (e.g., drawing a checkmark when `checked` is true).
- **Placeholder Logic:** Manages the display of ghost-text when the logic state is empty.

---

## 🗝️ Standard components

### `struct Input`
The primary text entry component.
- **Logic:** Manages a `Signal<String>` and cursor position.
- **View:** Renders a focused field with optional masking.

### `struct Checkbox` & `struct Switch`
Binary state toggles.
- **Logic:** Manages a `Signal<bool>`.
- **View:** Renders a square box (Checkbox) or a rounded slider (Switch).

---

## 🎨 Styling Standard
All form elements implement `Stylable`, allowing for deep visual customization while preserving their reactive logic.

```rust
Input::new(name_signal, "Username")
    .style(w(300.0))
```
