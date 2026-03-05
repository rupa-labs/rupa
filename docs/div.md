# Div Component

The `Div` is a generic **Semantic Component** used for grouping and layout within `Section` or other containers.

## 📐 Usage

Use `Div::new()` to create a general-purpose box for nesting content.

```rust
let button_group = Div::new()
    .style(Style::new()
        .p(8.0)
        .rounded(4.0)
        .bg(Color::Zinc(800)))
    .child("Confirm")
    .child("Cancel");
```

## 🛠 Features
- **Generic Box Model**: Supports full `Style` utilities (Padding, Margin, Rounded).
- **Flexible Children**: Can hold multiple child components or strings.
- **Utility-First**: Styled via the centralized `Style` object to stay **DRY**.

## 💡 Best Practices
- Use `Div` for small groupings, buttons, input wrappers, or layout cells.
- Prefer `Section` for high-level UI landmarks.
