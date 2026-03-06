# Module: TUI Runner (`tui/mod.rs`) 🖥️🏃

The TUI Runner is the platform shell for terminal environments. It replaces the window-based loop with a polling system optimized for character-based interfaces.

---

## 🧠 Internal Anatomy

### 1. Polling Lifecycle
- **Mechanism:** A while-loop that uses `poll_event`. 
- **Efficiency:** The loop remains idle until a terminal signal (Key, Mouse, or Resize) is received, keeping CPU usage near zero when the app is inactive.

### 2. Composition Shell
- **Composition:** Wraps **`PlatformCore`** (L1), **`TerminalInterface`** (L1), and **`TuiRenderer`** (L2).
- **Responsibility:** Maps terminal cell coordinates to the framework's logical Vec2 system.

---

## 🗝️ Logic & Flow

### Redraw Handling
1. Calls `self.core.compute_layout()`.
2. Triggers `root.paint()` using the `TuiRenderer`.
3. Calls `renderer.present()` to flush ANSI codes to the stdout.

### Input Decoding
Translates Crossterm enums into standardized `InputEvent` variants.

---

## 🔄 Interaction Flow
- **L1 (HAL) -> L1 (Terminal):** Manages raw mode and alternate screen setup.
- **L1 (HAL) -> L2 (TUI Renderer):** Drives the character-based painting process.
