# Interactivity, SVG & Accessibility

Rupaui provides high-level control over user interaction, vector graphics styling, and accessibility standards, ensuring your artisan UI is both functional and inclusive.

## 🖱 Interactivity

Control how users interact with elements and how the system provides feedback.

- `.accent(Color)`: Set the accent color for UI controls (checkboxes, radios).
- `.caret(Color)`: Set the color of the text insertion cursor.
- `.cursor(Cursor)`: Change the mouse cursor type (Pointer, Wait, Move, etc.).
- `.pointer_events(PointerEvents)`: Control if an element responds to mouse/touch events.
- `.user_select(UserSelect)`: Control if text can be selected by the user.
- `.scroll_smooth()`: Enable smooth scrolling behavior.
- `.resize(Resize)`: Allow users to resize an element (Horizontal, Vertical, Both).

```rust
Style::new()
    .cursor(Cursor::Pointer)
    .user_select(UserSelect::None)
    .accent(Color::Amber(500))
```

---

## 📐 SVG Styling

Specialized utilities for vector graphics (applied when the component is an SVG element).

- `.fill(Color)`: Set the interior color of the shape.
- `.stroke(Color)`: Set the outline color.
- `.stroke_w(f32)`: Set the outline thickness.

```rust
Style::new()
    .fill(Color::Indigo(600))
    .stroke(Color::White(1.0))
    .stroke_w(2.0)
```

---

## ♿ Accessibility

Utilities for ensuring the UI adapts to system accessibility settings.

- `.forced_color_adjust(ForcedColorAdjust)`: Control if the element's colors are overridden by high-contrast themes (Auto, None).

---

## 🗝 Key Types
- **Cursor**: `Default`, `Pointer`, `Wait`, `Text`, `Move`, `Help`, `NotAllowed`, `Crosshair`, `Grab`, `Grabbing`, and standard resize directions.
- **UserSelect**: `Auto`, `None`, `Text`, `All`.
- **PointerEvents**: `Auto`, `None`, `All`, and SVG-specific visible variants.
