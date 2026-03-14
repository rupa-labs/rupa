# `rupa-terminal` 📟

**The CLI Showroom.** This crate provides the **Adapters & Infrastructure** (Tier 3) to render Rupa applications directly inside the terminal using true-color ANSI and Crossterm.

---

## 🏛️ Architectural Role
- **Tier**: Tier 3 (Showrooms)
- **Identity**: The Finished Showroom (Adapters & Infrastructure)
- **Responsibility**: Translates VNode patches into optimized terminal character grids without screen flickering.

## 🛠️ Infrastructure Backends
- **Terminal Control**: `crossterm`
- **Event Polling**: Native TTY streams
- **Layout**: `taffy`

## 🚀 Usage

```rust
use rupa_terminal::prelude::*;

fn main() {
    TerminalRunner::new(App::new("My CLI App").root(MyComponent))
        .run()
        .unwrap();
}
```
