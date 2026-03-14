# Core Transitions ✨

Core transitions define how properties change over time using easing curves and duration-based logic.

---

## 🏗️ Usage
Apply a transition to any stylable property to ensure smooth visual changes.

### API Methods
| Method | Description | Example |
| :--- | :--- | :--- |
| **`.transition(props)`** | Marks properties for animation. | `.transition("all")` |
| **`.transition_duration(ms)`** | Sets the time taken to complete. | `.transition_duration(300)` |
| **`.transition_delay(ms)`** | Sets the wait time before starting. | `.transition_delay(100)` |
| **`.transition_repeat(n)`** | Sets the number of loops (0 for infinite). | `.transition_repeat(2)` |

---

## 🚀 Example
```rust
Button::new("Pulse")
    .on_hover_style(scale(1.1))
    .transition_duration(200)
    .transition_repeat(0) // Infinite pulse on hover
```
