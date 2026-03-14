# `rupa-console` 🖥️

**The Terminal Atom.** This crate provides **Atoms** for low-level terminal infrastructure and aesthetic CLI primitives. It is the foundation for all TUI-based artisan experiences.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Sanitized input polling prevents escape-sequence injection attacks.
    - **Sustain (S2)**: High-level semantic logging (Success/Error) reduces CLI boilerplate.
    - **Scalable (S3)**: Double-buffered rendering primitives ensure flicker-free terminal updates.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Console`** | Global Facade. | Static methods for colored text, lines, and boxes. |
| **`Progress`** | Reactive Tracker. | Supports horizontal bars with percentage and label signals. |
| **`Text`** | ANSI Styling. | Semantic coloring: Info (Blue), Success (Green), Error (Red). |
| **`MockConsole`** | Testing Backend. | Captures all terminal output into a string buffer for tests. |

## 🚀 Usage

```rust
use rupa_console::Console;

// 1. Semantic output
Console::success("Artifact compiled successfully.");
Console::error("Access denied.");

// 2. Structural primitives
Console::draw_box("Project Stats", vec![
    "Crates: 36".to_string(),
    "Status: Healthy".to_string(),
]);

// 3. Progress bars
Console::draw_progress("Uploading", 0.45, 1.0);
```

## 🧪 Testing & Reliability
- **Output Capture**: `MockConsole` allows tests to verify that the correct messages were printed to the user without polluting the real stdout.
- **Cross-Platform**: Abstracted ANSI handling ensures that colors and boxes work on Windows (CMD/PowerShell) and Unix terminals.
- **TDD Support**: Terminal UI logic can be verified in headless CI environments.
