# Platform Integration: Terminal Runner (`terminal/runner.rs`) ⌨️

The **Terminal Runner** provides the execution shell for command-line applications (TUI). It manages the terminal's raw mode, alternative screen buffer, and keyboard/mouse input capture via **Crossterm**.

---

## 🏗️ Architecture

The Terminal Runner translates the terminal's character grid environment into Rupa Framework's agnostik UI ecosystem.

### Key Responsibilities
- **Terminal Lifecycle**: Sets up and restores the terminal state (Raw Mode, Alt Screen).
- **Polling Loop**: Manages a synchronous loop for capturing input and triggering frames.
- **Character Grid Resolution**: Translates pixel-based coordinate requests into character grid units.
- **Input Mapping**: Maps Crossterm `Event` to Rupa Framework's `InputEvent`.

---

## 🗝️ API & Usage

### Starting a Terminal App
To run an application in the terminal, use the `run_terminal()` method:

```rust
App::new("CLI Tool")
    .run_terminal();
```

### Execution Flow
1. **Bootstrap**: Sets up the terminal via `TerminalInterface`.
2. **Main Loop**: Polls for keyboard and mouse events.
3. **Paint Phase**: Invokes the `TuiRenderer` to draw characters onto the terminal buffer.
4. **Restore**: Ensures the terminal is returned to its original state upon exit.

---

## 🛡️ Dependency Inversion (TerminalInterface)

Direct interactions with `crossterm` are encapsulated in `TerminalInterface` (`src/platform/terminal/terminal.rs`). This isolation ensures that the runner logic focuses on orchestration, not terminal-specific command handling.
