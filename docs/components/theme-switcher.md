# ThemeSwitcher Component

The `ThemeSwitcher` is a specialized semantic component that provides a standardized interface for users to toggle between application theme modes (**Dark**, **Light**, and **System**).

## 📐 Basic Usage

Place the `ThemeSwitcher` anywhere in your UI, typically in a Header or Sidebar.

```rust
use rupaui::elements::ThemeSwitcher;

let switcher = ThemeSwitcher::new();
```

---

## 🏗 How it Works

1.  **State Integration**: The component reads the current `mode` from the global `Theme`.
2.  **Automatic Cycling**: When clicked, it automatically cycles through the modes in this order: `Dark` -> `Light` -> `System` -> `Dark`.
3.  **Global Update**: It triggers a global theme update, which causes all theme-aware components (like `Brand` with theme-specific logos) to re-render.

---

## 🎨 Custom Styling

You can style the switcher like any other component.

```rust
ThemeSwitcher::new()
    .style((
        rounded(999.0), // Pill shape
        px(12.0),
        py(6.0)
    ));
```

## 🗝 Key Features
- **Plug & Play**: No manual state management required; it works out of the box with the global `Theme`.
- **User Preference**: Makes it easy to implement the standard modern "Dark Mode" toggle.
- **Visual Feedback**: Provides immediate visual cues about the active theme mode.
