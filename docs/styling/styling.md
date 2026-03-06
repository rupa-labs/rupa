# Styling & UI Utilities

Rupaui provides a comprehensive **Utility-First** styling system. By leveraging the `StyleModifier` trait and functional utilities, you can compose complex layouts and visual effects with high-performance Rust primitives.

## 📐 Box Model Utilities

Standard utilities for controlling the space and size of components. These are functional utilities that can be passed as a single modifier or in a tuple.

### Padding & Margin
- `p(val)` / `m(val)`: Sets padding/margin for all sides.
- `px(val)` / `py(val)`: Sets horizontal or vertical padding/margin.

### Dimensions
- `w(f32)` / `h(f32)`: Sets a fixed width or height.
- `w_full()` / `h_full()`: Sets width or height to 100%.

```rust
use rupaui::utils::{w, h, p, m};

// Used in a component
Button::new("Click Me")
    .style((
        w(200.0),
        h(100.0),
        p(16.0),
        m(8.0)
    ))
```

---

## 🏗 Layout & Positioning

Rupaui is optimized for the **Taffy** layout engine, supporting Flexbox and Grid.

### Display & Direction
- `flex()` / `grid()`: Sets the display mode.
- `row()` / `col()`: Sets the flex direction.
- `gap(f32)`: Sets the spacing between children.

### Alignment
- `justify_center()` / `justify_between()`: Aligns children along the main axis.
- `items_center()`: Aligns children along the cross axis.

### Positioning
- `relative()`: Default positioning.
- `absolute()`: Positions the element relative to its nearest positioned ancestor.

```rust
use rupaui::utils::{flex, col, gap, justify_center, items_center};

Section::new("Hero")
    .style((
        flex(),
        col(),
        gap(10.0),
        justify_center(),
        items_center()
    ))
```

---

## 🎨 Visual Effects

Advanced visual utilities optimized for **WGPU** GPU rendering.

### Background & Borders
- `bg(Color)`: Sets the background color.
- `rounded(f32)`: Sets the corner radius for all corners.

### Hover & State Modifiers
Rupaui supports state-based modifiers using utilities like `hover()` or `active()`.

```rust
use rupaui::utils::{bg, rounded, hover, active, scale};

Button::new("Interactive")
    .style((
        bg(Color::Indigo(500)),
        rounded(8.0),
        hover(bg(Color::Indigo(600))),
        active(scale(0.95, 0.95, 1.0))
    ))
```

---

## 🎨 The Artisan Color System

The Artisan Palette consists of **380 procedural colors** (20 bases x 19 shades) calculated using **OKLCH** for perceptual uniformity.

### Color Utilities
- `.a(f32)`: Applies transparency to any color variant.

```rust
Color::Blue(500).a(0.5) // Blue 500 at 50% opacity
```
