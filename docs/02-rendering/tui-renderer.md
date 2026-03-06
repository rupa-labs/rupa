# Module: TUI Renderer (`tui/mod.rs`) ⌨️

The TUI Renderer is the terminal-based implementation of the visual pipeline. It translates geometric coordinates into a grid of characters and ANSI colors.

---

## 🧠 Internal Anatomy

### 1. Cell Buffer (The Framebuffer)
The renderer maintains a 1D vector of `TuiCell` structs, mapped to a 2D coordinate system. Each cell contains a character symbol and RGB colors for both foreground and background.

### 2. Double Buffering & Diffing
- **Mechanism:** It stores the `prev_buffer`. 
- **Optimization:** During `present()`, it only prints ANSI escape codes for cells that have changed compared to the previous frame. This eliminates terminal flickering and reduces bandwidth.

---

## 🗝️ API Anatomy

- `new(w, h)`: Allocates the virtual character grid.
- `draw_rect()`: Uses Unicode box-drawing characters (`┌`, `─`, `┐`) to "paint" outlines into the buffer.
- `present()`: Flushes the diff-calculated buffer to the terminal `stdout`.

---

## 🔄 Interaction Flow
- **L2 (TUI Renderer) -> L1 (Terminal):** Sends raw ANSI sequences to the active terminal stdout.
