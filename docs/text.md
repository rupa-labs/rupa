# Text Component

The `Text` is a specialized **Semantic Component** for displaying content with full typography control.

## 📐 Usage

Use `Text::new()` to render content with full **Utility-First** styling.

```rust
Text::new("The Artisan's Atelier")
    .style(Style::new()
        .size(24.0)
        .weight(FontWeight::Bold)
        .color(Color::Emerald(500))
        .align(TextAlign::Center)
        .tracking(1.5));
```

## 🛠 Typography Utilities
- **Size**: `.size(f32)`
- **Weight**: `.weight(FontWeight)`
- **Font Family**: `.font(String)`
- **Color**: `.color(Color)`
- **Alignment**: `.align(TextAlign)`
- **Line Height**: `.leading(f32)`
- **Letter Spacing**: `.tracking(f32)`

## 💡 Best Practices
- Keep content semantic: use `Text` for all UI copy.
- Leverage the **Artisan Palette** (e.g., `Color::Zinc(400)`) for consistent color aesthetics.
