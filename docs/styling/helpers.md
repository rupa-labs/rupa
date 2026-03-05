# Helper Utilities & Components

Helpers in Rupaui provide specialized tools for common UI patterns, mapping directly to Bootstrap's helpers but optimized for Rust's type system and performance.

## 🛠 Semantic Helper Components

### Stacks (`HStack` & `VStack`)
Quickly stack elements horizontally or vertically with consistent spacing.

```rust
VStack::new()
    .gap(10.0)
    .child(Box::new(Text::new("Item 1")))
    .child(Box::new(Text::new("Item 2")));
```

### `Vr` (Vertical Rule)
Create a vertical line separator.

### `Ratio` (Aspect Ratio)
Force a specific aspect ratio on a container.

```rust
Ratio::new(16.0 / 9.0)
    .child(Box::new(Div::new().bg("slate-900")));
```

---

## ⚡ Style Helpers (Utility-First)

- `.clearfix()`: Clear floated content within a container.
- `.focus_ring()`: Apply a standardized focus indicator.
- `.stretched_link()`: Make a link cover the entire area of its closest positioned parent.
- `.truncate()`: Limit text to a single line with an ellipsis (`...`).
- `.visually_hidden()`: Hide an element visually while keeping it accessible to screen readers.

```rust
Text::new("Long artisan description that needs to be cut off")
    .style(truncate());
```

## 🗝 Practical Uses
- Use `VStack` for toolbars and sidebar lists.
- Use `Ratio` for the main pixel canvas or preview window.
- Use `visually_hidden` for ARIA labels that don't need a visual representation.
