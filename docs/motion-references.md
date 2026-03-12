# Animation & Motion Reference ✨

Motion is part of the Rupa Framework DNA. This document details how to create fluid, reactive animations that respond to state changes and user interactions.

---

## 🏗️ The `Motion` System
Animations in Rupa Framework are applied via styling modifiers. A `Motion` object defines how a property should transition between states.

### Transition Types
| Type | Logic | Description |
| :--- | :--- | :--- |
| **`Transition`** | Duration-based | Animates values over time using `Easing` curves. |
| **`Spring`** | Physics-based | Realistic motion with `mass`, `stiffness`, and `damping`. |

### Easing Functions
Support for standard curves: `Linear`, `Quad`, `Cubic`, `Expo`, `Back`, `Elastic` (In, Out, InOut).

---

## 🛠️ Usage Patterns

### 1. Duration-based Transitions
Apply a motion modifier to a component using `TransitionConfig`.

```rust
Text::new("Fade Me")
    .transition(Duration::from_millis(300), Easing::CubicInOut)
```

### 2. Spring Animations
Physics-based reactive animations linked to `Signal` updates.

```rust
let width = Signal::new(100.0);

Div::new()
    .w(width.get())
    .spring() // Default spring config
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
Rupa Framework's motion system is designed for high efficiency:
- **VNode Integrated**: Motion states are tracked across VNode diffing, ensuring smooth transitions even when the component tree structure changes.
- **GPU Accelerated**: Heavy property interpolation (Colors, Transforms) is handled by custom shaders in the `rupa-engine`.
- **Frame-Independent**: Animations use a high-precision delta time to ensure smoothness on any refresh rate (60Hz, 120Hz, etc.).
