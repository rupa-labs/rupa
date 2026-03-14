# Console Component Reference 📚

This document defines the standardized console components available in the `rupa-cli` for creating consistent and high-quality terminal outputs.

---

## 🏛️ Typography Components

Used for displaying semantic text with specific color coding and icons.

| Method | Level | Color | Icon | Description |
| :--- | :--- | :--- | :--- | :--- |
| `Console::text()` | Plain | White | - | Standard output message. |
| `Console::info()` | Info | Blue | ℹ | Detailed context or hints. |
| `Console::success()` | Success | Green | ✔ | Confirmation of a successful task. |
| `Console::warning()` | Warning | Yellow | ⚠ | Non-critical issues or alerts. |
| `Console::error()` | Error | Red | ✖ | Critical failures or system errors. |

---

## 🏗️ Layout Components

Used for structuring and organizing console output.

| Method | Component | Description |
| :--- | :--- | :--- |
| `Console::draw_line()` | Horizontal Line | A subtle separator for grouping output sections. |
| `Console::draw_box()` | Content Box | Wraps a title and multiple lines of text in an aesthetic border. |

---

## 📊 Feedback Components

Used for displaying the status of long-running operations.

| Method | Component | Description |
| :--- | :--- | :--- |
| `Console::draw_progress()` | Progress Bar | A reactive bar showing the percentage completion of a process. |

---

## 🚀 Usage Example

```rust
use rupa_cli::console::Console;

fn main() {
    Console::draw_box("Project Build", vec![
        "Target: Web (WASM)".to_string(),
        "Mode: Production".to_string(),
    ]);
    
    Console::draw_progress("Compiling", 75.0, 100.0);
    Console::success("Build complete!");
}
```
