# Module: TUI Terminal Interface (`tui/terminal.rs`) ⌨️

The Terminal Interface is the procedural wrapper for terminal-specific operations. it isolates the "dirty" work of Raw Mode and Buffer Management.

---

## 🧠 Internal Anatomy

### 1. Terminal Handshake
Implements the sequence to switch the current shell into a full-screen interactive mode:
1. `enable_raw_mode()`: Disables echo and line-buffering.
2. `EnterAlternateScreen`: Swaps to a clean buffer.
3. `EnableMouseCapture`: Requests the terminal to report pointer events.

### 2. Event Polling
Wraps the lower-level polling logic to provide a non-blocking way to check for hardware signals.

---

## 🗝️ API Anatomy

- `setup()`: Prepares the terminal environment.
- `restore()`: Ensures the terminal is returned to a usable state even if the application panics.
- `get_size()`: Fetches the current grid dimensions from the OS.

---

## 🔄 Interaction Flow
- **Platform Runner -> Terminal:** Calls `setup()` on boot and `restore()` on exit.
- **Terminal -> OS:** Directly communicates via ANSI sequences and syscalls.
