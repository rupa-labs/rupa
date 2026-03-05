# SVG Drawing & Vector Graphics

Rupaui features a high-level API for constructing vector graphics from scratch, powered by the `Vec2` math engine and semantic SVG components.

## 🏗 Drawing with `PathData`

The `PathData` builder allows you to construct complex paths using an intuitive, fluent API based on vectors.

### Basic Commands
- `.move_to(Vec2)`: Start a new sub-path.
- `.line_to(Vec2)`: Draw a straight line.
- `.bezier_to(ctrl1, ctrl2, end)`: Draw a cubic Bezier curve.
- `.quad_to(ctrl, end)`: Draw a quadratic Bezier curve.
- `.close()`: Close the current path.

```rust
use rupaui::utils::{PathData, Vec2};

let triangle = PathData::new()
    .move_to((0.0, 0.0))
    .line_to((100.0, 0.0))
    .line_to((50.0, 100.0))
    .close();
```

---

## 🎨 Semantic Components

### `SvgCanvas`
The root container for all vector elements. It defines the coordinate system and viewport.

### `Path`
Represents a single vector shape defined by `PathData`.

```rust
use rupaui::elements::{SvgCanvas, Path};
use rupaui::utils::{fill, stroke};

let icon = SvgCanvas::new()
    .child(Box::new(
        Path::new()
            .data(triangle)
            .style((
                fill(Color::Indigo(500)),
                stroke(Color::White(1.0), 2.0)
            ))
    ));
```

---

## ⚡ Integration with `Vec2`

Because `PathData` uses `Vec2` internally, you can use vector math to calculate points dynamically.

```rust
let center = Vec2::new(50.0, 50.0);
let radius = 40.0;

let mut circle_path = PathData::new().move_to((center.x + radius, center.y));

for i in 1..=360 {
    let point = center + Vec2::new(1.0, 0.0).rotate(i as f32) * radius;
    circle_path = circle_path.line_to(point);
}
```

## 🗝 Key Types
- **PathData**: The builder for vector instructions.
- **Vec2**: High-precision 2D vector for coordinates.
