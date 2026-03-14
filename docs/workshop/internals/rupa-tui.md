# `rupa-tui` 🖥️

**The Terminal Orquestrator.** Translates the VNode tree into high-performance terminal characters.

## 🛠️ Key Features

- **`TerminalRenderer`**: A high-speed TUI renderer with RGB support.
- **Double Buffering**: Optimizes screen updates to prevent flickering.
- **Layout Mapping**: Translates pixel-based layout logic to character grids.

## 🚀 Usage

```rust
use rupa_tui::TerminalRenderer;

// Initialize the renderer
let mut renderer = TerminalRenderer::new(80.0, 24.0);

// Clear and move cursor
renderer.clear_screen();
renderer.move_cursor(0, 0);

// Draw primitives
renderer.draw_text("Rupa TUI", 0.0, 0.0, 80.0, 1.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
renderer.present();
```
