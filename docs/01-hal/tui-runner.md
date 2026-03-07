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

## 🌟 High-Performance Terminal Experience

### Event-Driven Efficiency
The TUI backend is built for zero-latency interactions with minimal resource footprint. 
- **Idle Optimization:** The runner remains in a suspended state when no input is detected, ensuring 0% CPU usage.
- **Immediate Response:** OS-level events are captured via dedicated threading to guarantee a fluid UI experience.

### Modern Aesthetics
Rupaui brings modern design principles to the terminal:
- **TrueColor Support:** Full 24-bit color depth for rich gradients and subtle theme transitions.
- **Theme Awareness:** Automatic detection of terminal background colors to adapt UI contrasts dynamically.

### Resilient Shell Management
The framework prioritizes terminal integrity:
- **Panic Protection:** Automatic restoration of the terminal's raw mode and alternate screen even in the event of an application crash.
- **Grid Mapping:** Intelligent translation between pixel-based logic and character-grid physics for unified component behavior.


