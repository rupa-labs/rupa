# Animation & Motion Reference ✨

Motion is part of the Rupaui DNA. This document details how to create fluid, reactive animations that respond to state changes and user interactions.

---

## 🏗️ The `Motion` System
Animations in Rupaui are applied via styling modifiers. A `Motion` object defines how a property should transition between states.

### Transition Types
| Type | Description |
| :--- | :--- |
| **Linear** | Consistent speed throughout the transition. |
| **EaseIn / EaseOut** | Smooth acceleration or deceleration. |
| **Spring** | Physics-based motion with bounce and stiffness. |

---

## 🛠️ Usage Patterns

### 1. Simple Transitions
Apply a motion modifier to a component to animate property changes.

```rust
Text::new("Hover me")
    .style(hover(bg(Color::Blue)))
    .style(motion(Duration::from_millis(300), Easing::InOut))
```

### 2. Reactive Animations
Link animations directly to `Signal` updates. When the signal changes, the UI transitions smoothly to the new state.

```rust
let width = Signal::new(100.0);

Div::new()
    .w(width.get())
    .style(motion(Duration::from_millis(500), Easing::Spring))
```

---

## 🎨 Motion Properties
The following properties can currently be animated:
- **Colors**: Background, Text, and Border colors.
- **Sizing**: Width and Height.
- **Spacing**: Padding and Margin.
- **Layout**: Opacity and Transforms (Translate, Scale, Rotate).

---

## 🚀 Performance
Rupaui's motion system is designed for high efficiency:
- **GPU Accelerated**: Heavy interpolation is handled by shaders where possible.
- **Frame-Independent**: Animations use a high-precision delta time to ensure smoothness on both 60Hz and 144Hz monitors.
