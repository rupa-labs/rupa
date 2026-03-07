# Module: Vector Graphics (`svg.rs`) 🎨

This module provides components for rendering mathematical paths and iconography directly on the GPU.

---

## 🏗️ Components

### `struct Svg`
A container for complex vector paths.
- **Logic:** Manages a collection of `Path` objects.
- **View:** Iterates through paths and commands the Vector Engine (L2).

### `struct Icon`
A simplified component for rendering standardized symbols.
- **Logic:** Tracks name and size.
- **View:** Paints high-resolution symbols via SDF shaders.

---

## 🗝️ Semantic API

- `.add_path(Path)`: Appends geometry to the SVG canvas.
- `.size(f32)`: Controls the icon's scale.

## 💻 Usage Example

```rust
Icon::new("heart")
    .size(24.0)
    .color([1.0, 0.0, 0.0, 1.0])
```
