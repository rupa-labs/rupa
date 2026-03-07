# Module: Theme Switcher (`theme_switcher.rs`) 🌓

This module provides an interactive control for toggling between Light and Dark visual modes.

---

## 🏗️ Components

### `struct ThemeSwitcher`
A specialized button that communicates directly with the Global Theme Engine (L9).
- **Logic:** Triggers the `Theme::cycle_mode()` static method.
- **View:** Changes its own visual icon based on the current `ColorMode`.

---

## 💻 Usage Example

```rust
Navbar::new()
    .end(Box::new(ThemeSwitcher::new()))
```
