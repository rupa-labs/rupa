# Theming & Design Tokens

The `Theme` module in Rupaui acts as a centralized store for **Design Tokens**. It is inspired by CSS Variables but is fully type-safe and optimized for Rust.

## 📐 Why use Themes?
- **Consistency**: Ensure colors, spacing, and styles are identical across all components.
- **Maintainability**: Update a single token (e.g., `primary`) to change the look of the entire application.
- **Dynamic Switching**: Easily implement Dark Mode or custom user themes.

---

## 🚀 Defining Tokens

You can update the global theme at any time (e.g., during app initialization).

```rust
use rupaui::utils::{Theme, Color, Style};

Theme::update(|t| {
    // 1. Define custom colors
    t.colors.insert("brand-gold".into(), Color::Amber(400));
    
    // 2. Define custom spacing
    t.spacing.insert("compact".into(), 4.0);
    
    // 3. Define complete Style presets
    let card_style = Style::new()
        .p(16.0)
        .bg(Color::Slate(800))
        .rounded(8.0);
    t.styles.insert("card".into(), card_style);
});
```

---

## 🎨 Using Tokens in Components

The `Theme` class provides static accessors to retrieve your tokens.

```rust
// Using a theme color
let title_style = Style::new()
    .color(Theme::color("accent"))
    .p(Theme::space("md"));

// Using a complete style preset
let card = Div::new().style(Theme::style("card"));
```

---

## 🛠 Default Artisan Theme

Rupaui comes with a pre-configured `default_artisan()` theme:

- **Colors**: `primary`, `secondary`, `accent`, `background`, `surface`, `text`.
- **Spacing**: `xs` (4), `sm` (8), `md` (16), `lg` (24), `xl` (32).

## 💡 Adding Custom Utilities

Since the `Theme` struct uses `HashMap` for its tokens, you can store any string-based custom utility:

```rust
Theme::update(|t| {
    t.custom.insert("transition-speed".into(), "200ms".into());
});

let speed = Theme::current().custom.get("transition-speed");
```
