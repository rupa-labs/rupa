# `rupa-canvas` 🎨

**The Media Port.** This crate provides **Atoms** for low-level drawing commands and custom graphics. It serves as the universal language for hardware-accelerated painting.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Command-based drawing prevents direct, unsafe GPU memory access.
    - **Sustain (S2)**: Simplified API for common primitives (Rect, Circle, Text) reduces boilerplate.
    - **Scalable (S3)**: Designed for efficient batching in Tier 3 renderers (e.g., WGPU).

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Command`** | Drawing Instruction. | Atomic variants for `Rect`, `Circle`, `LineTo`, `Text`, and `Clip`. |
| **`Canvas`** | Drawing Context. | Accumulates commands reactively for the next frame. |
| **`DrawTarget`** | The Canvas Port. | Trait defining the execution of a command buffer. |
| **`MockCanvas`** | Testing Backend. | Records all drawing commands for structural verification. |

## 🚀 Usage

```rust
use rupa_canvas::{Canvas, Command, MockCanvas};
use rupa_base::{Rect, Color};

// 1. Setup a reactive canvas
let mut canvas = Canvas::new();

// 2. Queue drawing commands
canvas.draw_rect(Rect::new(0.0, 0.0, 100.0, 100.0), Color::BLUE);
canvas.draw_text("Artisan", Vec2::new(10.0, 10.0), 16.0, Color::WHITE);

// 3. Execution (Handled by Showroom)
let target: Arc<dyn DrawTarget> = Arc::new(MockCanvas::new());
target.execute(canvas.commands().to_vec());
```

## 🧪 Testing & Reliability
- **Command Verification**: `MockCanvas` allows tests to assert that specific shapes or text were drawn at precise coordinates.
- **TDD Support**: Visual logic can be verified in headless environments without a physical window.
- **Hardware Agnostic**: The core Atom doesn't know about WGPU or OpenGL; it only knows *what* to draw.
