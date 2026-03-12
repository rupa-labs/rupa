# `rupa-canvas` 🎨

**The Low-Level Drawing Port.** Agnostic primitives for custom painting and GPU-accelerated graphics.

## 🛠️ Key Features

- **`Command`**: Low-level drawing instructions (Rect, Circle, Path, Text).
- **`Canvas`**: A state container that accumulates drawing commands.
- **`DrawTarget`**: The Port for implementation by platform adapters (e.g., WGPU or HTML5 Canvas).

## 🚀 Usage

```rust
use rupa_canvas::{Canvas, Command};
use rupa_base::{Rect, Color};

// 1. Accumulate commands
let mut canvas = Canvas::new();
canvas.draw_rect(Rect::new(0.0, 0.0, 100.0, 100.0), Color::RED);

// 2. Platform adapter (Tier 3) will execute these
let commands = canvas.commands();
```
