# Theming & Customization

Rupaui provides a centralized system for customizing the visual identity of your application, mapping directly to Bootstrap's "Customize" concepts but with Rust's type safety.

## 🎨 Color Modes

Rupaui supports **Dark**, **Light**, and **System** color modes natively.

```rust
use rupaui::utils::{Theme, ColorMode};

Theme::update(|t| {
    t.mode = ColorMode::Light;
});
```

---

## 🏷 Semantic Variants

Variants provide a meaningful way to apply colors to components (`Primary`, `Danger`, `Success`, etc.).

### Default Mapping
- `Primary` -> Indigo 600
- `Success` -> Emerald 500
- `Danger` -> Red 500
- `Warning` -> Amber 400

### Customizing Variants
```rust
use rupaui::utils::{Theme, Variant, Color};

Theme::update(|t| {
    t.variants.insert(Variant::Primary, Color::Rose(600));
});
```

---

## 🗝 Design Tokens (CSS Variables)

The `Theme` module acts as a global store for tokens, similar to CSS Variables.

```rust
// Defining a token
Theme::update(|t| {
    t.colors.insert("brand-gold".into(), Color::Amber(400));
});

// Using a token in Style
let style = Style::new().bg(Theme::color("brand-gold"));
```

---

## ⚡ Optimization

Rupaui is designed for maximum efficiency in **WebAssembly**:
- **Design Tokens** are resolved at the component level, minimizing global state lookups.
- **Variants** are stored in fixed-size HashMaps for O(1) access time.
- **Static Resolution**: Whenever possible, use `Theme::current()` to batch multiple property lookups.
