# Module: TUI Renderer (`crates/rupa-engine/src/renderer/tui/renderer.rs`) ⌨️

The TUI Renderer is the terminal-based implementation of the visual pipeline in the Rupa Framework. It translates the agnostic `VNode` tree into a grid of characters and ANSI colors.

---

## 🧠 Internal Anatomy

### 1. Cell Buffer (The Virtual Framebuffer)
The renderer maintains a 1D vector of cells mapped to a 2D coordinate system. Each cell contains:
- **Character**: The Unicode symbol to display.
- **Style**: Perceptually uniform OKLCH colors for foreground and background.

### 2. Reconciliation & Patching
- **VNode Mapping**: During the **Patch Phase**, the TUI Renderer consumes instructions to update the character grid.
- **Double Buffering**: It compares the `new_buffer` with the `prev_buffer` to minimize stdout output.

---

## 🗝️ API Anatomy

- **`render_patch(patch)`**: The primary entry point. Translates VNode elements into Unicode box-drawing characters (`┌`, `─`, `┐`) or raw text strings.
- **`present()`**: Flushes the diff-calculated buffer to the terminal `stdout` using optimized ANSI escape sequences.
- **`resize(w, h)`**: Reallocates the internal buffer to match the new terminal terminal dimensions.

---

## 🔄 Integration

The TUI Renderer is active when the `tui` feature is enabled. It provides the visual surface for the **Terminal Runner**, allowing the same UI components to run seamlessly via SSH or in a minimal CLI environment.
