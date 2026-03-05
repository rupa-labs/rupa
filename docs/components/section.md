# Section Component

`Section` is a **Semantic Layout Component** used to define large structural regions of your interface.

## 📐 Usage

Use `Section::new()` with a descriptive name to create a semantic boundary for your layout.

```rust
let sidebar = Section::new("ToolPanel")
    .style(Style::new()
        .bg(Color::Slate(900))
        .w(300.0)
        .h_full())
    .child(Box::new(Div::new().child("Tools...")));
```

## 🛠 Features
- **Semantic Hierarchy**: Provides a named region for layout resolution.
- **Style Support**: Accepts a full `Style` object for utility-first decoration.
- **Children**: Accepts any object implementing the `Component` trait.

## 💡 Best Practices
- Use `Section` for high-level UI parts: `Header`, `Sidebar`, `MainContent`, `Footer`.
- Avoid nesting many `Sections`; use `Div` for sub-grouping.
