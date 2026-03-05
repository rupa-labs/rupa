# Background & Border Utilities

Rupaui provides a powerful suite of utilities for managing element backgrounds, borders, and outlines, with full support for modern CSS features.

## 🖼 Backgrounds

- `.bg(Color)`: Set the background color.
- `.bg_image(String)`: Set a background image URL or path.
- `.bg_size(BackgroundSize)`: Control background scaling (`Cover`, `Contain`, `Auto`).
- `.bg_repeat(BackgroundRepeat)`: Control image tiling.
- `.bg_pos(x, y)`: Set the background position.
- `.bg_clip(BackgroundClip)` / `.bg_origin(BackgroundOrigin)`: Control the painting area.

```rust
Style::new()
    .bg(Color::Slate(900))
    .bg_image("assets/pattern.png")
    .bg_size(BackgroundSize::Cover)
```

---

## 🧱 Borders

- `.border(width, style, color)`: Shorthand for setting all border properties.
- `.border_w(f32)`: Set border thickness.
- `.border_style(BorderStyle)`: Set border style (`Solid`, `Dashed`, `Dotted`, etc.).
- `.border_color(Color)`: Set border color.
- `.rounded(f32)`: Set the corner radius (Border Radius).

```rust
Style::new()
    .border(2.0, BorderStyle::Solid, Color::Indigo(500))
    .rounded(12.0)
```

---

## 🏎 Outlines

Outlines are drawn outside the border edge and do not take up space in the layout.

- `.outline(width, style, color)`: Shorthand for setting all outline properties.
- `.outline_w(f32)`: Set outline thickness.
- `.outline_offset(f32)`: Set the space between the border and the outline.

```rust
Style::new()
    .outline(2.0, BorderStyle::Dashed, Color::Amber(400))
    .outline_offset(4.0)
```

## 🗝 Key Types
- **BorderStyle**: `Solid`, `Dashed`, `Dotted`, `Double`, `Groove`, `Ridge`, `Inset`, `Outset`.
- **BackgroundSize**: `Auto`, `Cover`, `Contain`, `Length(f32, f32)`.
