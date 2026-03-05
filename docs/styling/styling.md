# Styling & UI Utilities

Rupaui provides a comprehensive **Utility-First** styling system. By leveraging the `Style` object, you can compose complex layouts and visual effects with high-performance Rust primitives.

## 📐 Box Model Utilities

Standard utilities for controlling the space and size of semantic components.

### Padding & Margin
- `.p(f32)` / `.m(f32)`: Sets padding/margin for all sides.
- `.px(f32)` / `.py(f32)`: Sets horizontal or vertical padding.

### Dimensions
- `.w(f32)`: Sets a fixed width.
- `.h(f32)`: Sets a fixed height.

```rust
Style::new()
    .w(200.0)
    .h(100.0)
    .p(16.0)
    .m(8.0)
```

---

## 🏗 Layout & Positioning

Rupaui is optimized for the **Taffy** layout engine, supporting Flexbox and Grid.

### Display & Direction
- `.flex()` / `.grid()`: Sets the display mode.
- `.row()` / `.col()`: Sets the flex direction.
- `.gap(f32)`: Sets the spacing between children.

### Alignment
- `.justify(JustifyContent)`: Aligns children along the main axis (Start, Center, End, SpaceBetween, etc.).
- `.items(AlignItems)`: Aligns children along the cross axis (Start, Center, End, Stretch).

### Positioning
- `.relative()`: Default positioning.
- `.absolute()`: Positions the element relative to its nearest positioned ancestor.

```rust
Style::new()
    .flex()
    .col()
    .gap(10.0)
    .justify(JustifyContent::Center)
    .items(AlignItems::Stretch)
```

---

## 🎨 Visual Effects

Advanced visual utilities optimized for **WGPU** GPU rendering.

### Background & Borders
- `.bg(Color)`: Sets the background color using the Artisan Palette.
- `.rounded(f32)`: Sets the corner radius for all corners.

### Opacity & Z-Index
- `.opacity(f32)`: Sets the transparency level (0.0 to 1.0).
- `.z(i32)`: Sets the stacking order of the element.

### Shadows
- `.shadow(x, y, blur, color)`: Adds a box shadow. Multiple shadows can be chained.

```rust
Style::new()
    .bg(Color::Slate(800))
    .rounded(8.0)
    .opacity(0.95)
    .shadow(0.0, 4.0, 15.0, Color::Black(0.4))
    .z(10)
```

---

## 🎨 The Artisan Color System

The Artisan Palette consists of **380 procedural colors** (20 bases x 19 shades) calculated using **OKLCH** for perceptual uniformity.

### Color Utilities
- `.a(f32)` / `.opacity(f32)`: Applies transparency to any color variant.

```rust
Color::Blue(500).a(0.5) // Indigo 500 at 50% opacity
```
