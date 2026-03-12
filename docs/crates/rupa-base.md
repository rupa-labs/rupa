# `rupa-base` 🧱

**The Standard Types.** Foundational data structures used across all tiers of the framework.

## 🛠️ Key Features

- **`Color`**: High-precision RGBA and OKLCH color management.
- **`Vec2`**: 2D vector for spatial and layout calculations.
- **`Rect`**: Bounding box / Rectangle logic.
- **`Id`**: Lightweight, unique 64-bit identification system.

## 🚀 Usage

```rust
use rupa_base::{Color, Vec2, Rect, Id};

// 1. Color with OKLCH (Perceptual Uniformity)
let primary = Color::oklch(0.8, 0.2, 200.0, 1.0);

// 2. Spatial types
let pos = Vec2::new(100.0, 50.0);
let bounds = Rect::new(0.0, 0.0, 800.0, 600.0);

if bounds.contains(pos) {
    println!("Inside bounds!");
}

// 3. Unique ID
let my_id = Id::next();
println!("Generated: {}", my_id); // "rupa-1"
```
