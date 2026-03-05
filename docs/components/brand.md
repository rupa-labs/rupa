# Brand Component

The `Brand` component is a specialized semantic element for application identity, supporting logos, names, and built-in responsiveness.

## 📐 Theme-Aware Logos

Since logos are static images, they might not look good on both light and dark backgrounds. Rupaui allows you to define variants for each theme mode.

```rust
use rupaui::elements::Brand;

let my_brand = Brand::new("Rupa Editor")
    .logo(
        "assets/logo-default.png", // Fallback
        Some("assets/logo-light.png".into()), // For Light Mode
        Some("assets/logo-dark.png".into())   // For Dark Mode
    );
```

---

## 📱 Responsiveness

The `Brand` component can automatically switch to "Logo Only" mode on small screens.

- `.collapse_at(Breakpoint)`: Define the threshold for hiding the text label.

```rust
Brand::new("Artisan's Atelier")
    .logo("logo.png", None, None)
    .collapse_at(Breakpoint::Md); 
```

---

## 🗝 Key Features
- **Dynamic Switching**: Automatically selects the appropriate logo based on `Theme::current().mode`.
- **Identity First**: Ensures brand consistency across different system appearance settings.
- **A11y Integrated**: Uses ARIA roles to ensure the identity is accessible to screen readers.
