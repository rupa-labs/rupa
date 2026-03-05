# Vector Math (`Vec2`)

Rupaui includes a high-performance 2D vector library designed for SVG path calculations, layout transformations, and coordinate normalization.

## 📐 The `Vec2` Struct

`Vec2` is a lightweight, `Copy`-able struct consisting of `x` and `y` float values.

### Basic Operations
Supports standard Rust operator overloading for arithmetic.

```rust
use rupaui::utils::Vec2;

let a = Vec2::new(10.0, 5.0);
let b = Vec2::new(2.0, 2.0);

let sum = a + b;      // Vec2(12.0, 7.0)
let diff = a - b;     // Vec2(8.0, 3.0)
let scaled = a * 2.0; // Vec2(20.0, 10.0)
```

---

## 🏗 Advanced Math

- `.length()`: Calculate the magnitude of the vector.
- `.normalize()`: Returns a vector with length 1.0 in the same direction.
- `.distance(other)`: Calculate Euclidean distance between two points.
- `.dot(other)`: Calculate the dot product.
- `.lerp(other, t)`: Linearly interpolate between two vectors.
- `.rotate(degrees)`: Rotate the vector around the origin (0,0).
- `.angle()`: Get the angle of the vector in degrees.

```rust
let start = Vec2::new(0.0, 0.0);
let end = Vec2::new(100.0, 100.0);

// Smoothly transition between points
let midpoint = start.lerp(end, 0.5); 
```

## 🗝 Practical Uses
- **SVG Paths**: Calculating Bezier control points or arc segments.
- **Input Normalization**: Mapping mouse coordinates to canvas pixels.
- **Physics**: Handling velocity or force vectors in interactive UI elements.
