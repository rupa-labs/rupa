# `rupa-console` 🖥️

**The Terminal Material.** Low-level utilities for creating high-quality, standardized terminal output.

## 🛠️ Key Features

- **`Text`**: ANSI-colored semantic logging (Info, Success, Warning, Error).
- **`Layout`**: ASCII/ANSI layout helpers like lines and boxes.
- **`Progress`**: Reactive terminal progress bars.
- **`Console`**: A facade aggregating all terminal utilities.

## 🚀 Usage

```rust
use rupa_console::Console;

// 1. Semantic logging
Console::success("System initialized.");
Console::error("Database connection failed.");

// 2. Draw UI elements
Console::draw_line();
Console::draw_box("App Stats", vec![
    "Uptime: 2h".to_string(),
    "Active Users: 15".to_string(),
]);

// 3. Progress bars
Console::draw_progress("Compiling", 75.0, 100.0);
```
