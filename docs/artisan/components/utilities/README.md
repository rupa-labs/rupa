# Style Utilities 🎨

Style utilities are the "brushes and chisels" of the Rupa Framework. They provide a fluent, chainable API to modify the appearance and layout of any UI material.

---

## 🏗️ The Fluent Philosophy
Rupa uses a functional approach to styling. Instead of writing external CSS or complex style objects, you apply modifications directly to your components using semantic methods.

```rust
Button::new("Save")
    .p(Scale::Md)
    .bg_primary()
    .rounded(Scale::Sm)
```

## 📐 Unified Scaling
All utilities are integrated with the **Artisan Scale** system, ensuring that your application maintains a consistent visual rhythm across all platforms.

---

## 🧭 Explore Materials
For a complete list of available style modifiers and their technical definitions, please refer to the:

👉 **[Style Utility Registry](./utility-references.md)**
