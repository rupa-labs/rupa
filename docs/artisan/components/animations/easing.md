# Easing Functions 📈

Easing functions define the rate of change of a transition over time, making animations feel more natural.

---

## 🏗️ Core Easings
Rupa provides standard industry curves for duration-based transitions.

| Method | Description | Best For |
| :--- | :--- | :--- |
| **`.ease_in()`** | Starts slow, accelerates at the end. | Elements leaving the screen. |
| **`.ease_out()`** | Starts fast, decelerates at the end. | Elements entering the screen. |
| **`.ease_in_out()`** | Slow start and end, fast in the middle. | General property changes (Colors, Size). |
| **`.linear()`** | Constant speed. | Background patterns or infinite loops. |

---

## 🚀 Usage
Easings are applied to transitions to modify their "feel".

```rust
Text::new("Smooth Move")
    .transition_duration(500)
    .ease_in_out()
```
