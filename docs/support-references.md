# Support & Infrastructure Reference 🛠️

This document details the built-in support systems that power Rupaui's core logic, from mathematical utilities to reactive state management and accessibility.

---

## 📐 Math & Geometry
Foundational types for spatial calculations and vector manipulation.

### `struct Vec2`
A high-precision 2D vector used for layouts, SVG paths, and pointer coordinates.

| Method | Description | Example |
| :--- | :--- | :--- |
| `Vec2::new(x, y)` | Creates a new vector. | `Vec2::new(10.0, 20.0)` |
| `v.length()` | Returns the magnitude of the vector. | `v.length()` |
| `v.normalize()` | Returns a unit vector in the same direction. | `v.normalize()` |
| `v.lerp(other, t)` | Linearly interpolates between two vectors. | `v1.lerp(v2, 0.5)` |
| `v.rotate(deg)` | Rotates the vector by a given degree. | `v.rotate(45.0)` |
| `v.distance(other)` | Calculates the distance between two points. | `v1.distance(v2)` |

*Note: `Vec2` supports standard operator overloading (`+`, `-`, `*`, `/`).*

---

## ⚡ Reactivity (Signals)
Fine-grained reactivity system for efficient UI updates.

### `struct Signal<T>`
A thread-safe reactive primitive that notifies the framework to redraw when its value changes.

| Method | Description | Example |
| :--- | :--- | :--- |
| `Signal::new(val)` | Creates a new signal. | `let count = Signal::new(0);` |
| `.get()` | Returns a clone of the current value. | `count.get()` |
| `.set(val)` | Overwrites the value and triggers redraw. | `count.set(10)` |
| `.update(f)` | Modifies the value in-place and triggers redraw. | `count.update(\|v\| *v += 1)` |

### `struct Memo<T, D>`
A derived reactive value that only re-computes when its source signal changes.

```rust
let count = Signal::new(10);
let double = Memo::new(count.clone(), |v| v * 2);
```

---

## ♿ Accessibility (A11y)
Semantic metadata for Screen Readers and assistive technologies.

### `struct AccessibilityNode`
The container for semantic information provided via the `.accessibility()` component hook.

| Property | Description |
| :--- | :--- |
| `role` | The semantic purpose (e.g., `Button`, `Heading`). |
| `label` | The accessible name of the element. |
| `description` | A detailed hint or explanation. |
| `value` | The current textual value (e.g., for sliders). |

---

## 🎨 Advanced Color System
High-fidelity color manipulation using modern perceptual models.

### `enum Color`
| Variant | Description |
| :--- | :--- |
| **Oklch(l, c, h, a)** | Perceptually uniform color (Lightness, Chroma, Hue). |
| **Rgba(r, g, b, a)** | Standard Red, Green, Blue, Alpha. |
| **Semantic(name, a)** | Theme-aware color (e.g., "primary", "background"). |

---

## 🆔 Identity
Utilities for component tracking and stable referencing.

### `fn generate_id()`
Generates a unique, stable string ID for components. Essential for internal state tracking and debugging.
- **Format:** `rupa-{counter}`
- **Usage:** Typically handled automatically by component constructors.

---

## 🛡️ Debugging & Error Handling
Robust systems for framework diagnostics and graceful failure recovery.

### `enum RupauiError`
Centralized error classification for all framework subsystems.
- **`Layout`**: Failures in the geometry engine (Taffy).
- **`Renderer`**: GPU, Shader, or Surface-related issues.
- **`Platform`**: OS-level windowing or event loop failures.
- **`Component`**: Logic errors specific to a UI element.

### `struct DiagnosticCenter`
The central hub for error reporting. Users can hook into this via the `App` API.

### Usage Example
```rust
App::new("Artisan App")
    .debug(true) // Activates wireframes and event logging
    .on_error(|err| {
        eprintln!("Caught Framework Error: {}", err);
    })
```

