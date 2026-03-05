# Spacing & Sizing Utilities

Rupaui provides a flexible system for controlling the dimensions and spacing of semantic components, including support for CSS-like shorthands and logical properties.

## 📏 Sizing

Control the physical and logical dimensions of elements.

### Physical Sizing
- `.w(f32)` / `.h(f32)`: Set width and height.
- `.min_w(f32)` / `.max_w(f32)`: Set minimum and maximum width.
- `.min_h(f32)` / `.max_h(f32)`: Set minimum and maximum height.

### Logical Sizing (Writing-mode aware)
- `.inline_size(f32)`: Set size in the inline direction (maps to width in horizontal mode).
- `.block_size(f32)`: Set size in the block direction (maps to height in horizontal mode).
- Supports `.min_` and `.max_` prefixes for both.

```rust
Style::new()
    .w(500.0)
    .max_w(1000.0)
    .min_h(200.0)
    .inline_size(300.0)
```

---

## ↔️ Spacing (Margin & Padding)

Rupaui supports flexible input types for spacing, allowing you to set 1, 2, or 4 values at once.

### Input Formats
- `f32`: Sets all sides (e.g., `.p(10.0)`).
- `(f32, f32)`: Sets (Vertical, Horizontal) (e.g., `.p((8.0, 16.0))`).
- `(f32, f32, f32, f32)`: Sets (Top, Right, Bottom, Left) (e.g., `.p((10.0, 5.0, 15.0, 2.0))`).

### Shorthands
- `.m()` / `.p()`: Main margin and padding methods.
- `.mx()` / `.px()`: Horizontal only.
- `.my()` / `.py()`: Vertical only.

```rust
Style::new()
    .m(20.0)             // Margin 20 on all sides
    .mx(10.0)            // Override: Margin 10 on Left/Right
    .p((16.0, 32.0))     // Padding: 16 Top/Bottom, 32 Left/Right
```

## 💡 Best Practices
- Use **Logical Sizing** (`inline_size`) if you plan to support vertical writing modes in the future.
- Use **Tuple Shorthands** for padding to keep your style chains short and readable.
