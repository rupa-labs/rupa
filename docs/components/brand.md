# Brand Component

The `Brand` component is a specialized semantic element for application identity, supporting logos, names, and built-in responsiveness.

## 📐 Basic Usage

Create a brand with a name and an optional logo.

```rust
use rupaui::elements::Brand;

let my_brand = Brand::new("Rupa Editor")
    .logo("assets/rupa-logo.png");
```

---

## 📱 Responsiveness

The `Brand` component can automatically switch to "Logo Only" mode on small screens to save space.

- `.collapse_at(Breakpoint)`: Define the breakpoint where the text label should be hidden.

```rust
use rupaui::utils::Breakpoint;

Brand::new("Artisan's Atelier")
    .logo("logo.png")
    .collapse_at(Breakpoint::Md); // Shows only logo on Sm and Xs screens.
```

---

## ♿ Accessibility

By default, the `Brand` component uses the `Link` role, assuming it will navigate to the home page. It automatically includes an ARIA label.

## 🗝 Key Features
- **Responsive by Design**: Built-in logic for handling identity display on different viewports.
- **Identity First**: Combines vector/raster logos with semantic text labels.
- **Wasm Optimized**: Uses simple threshold checks for layout switching.
