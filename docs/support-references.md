# Support & Infrastructure Reference 🛠️

This document details the built-in support systems that power Rupa Framework's core logic, from mathematical utilities to reactive state management and accessibility.

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

---

## 🛡️ Debugging & Error Handling
Robust systems for framework diagnostics and graceful failure recovery.

### `enum Error`
Centralized technical error classification for all framework subsystems.
- **`Layout`**: Failures in the geometry engine (Taffy).
- **`Renderer`**: GPU, Shader, or Surface-related issues.
- **`Platform`**: OS-level windowing or event loop failures.
- **`Component`**: Logic errors specific to a UI element.
- **`Unsupported`**: Features not yet implemented or available on the current target.
- **`Reactivity`**: Out-of-sync states in the Signal/Memo engine.

### `struct DiagnosticCenter`
The central hub for error reporting. Users can hook into this via the `App` API.

### Usage Example
```rust
App::new("Artisan App")
    .on_error(|err| {
        match err {
            Error::Unsupported(f) => log::warn!("Feature {} is not yet supported", f),
            _ => log::error!("System Error: {}", err),
        }
    })
```

---

## 🏗️ Metadata Manifest
Types used to define the application's identity and platform-specific behaviors.

### `struct AppMetadata`
The central manifest for application properties.

| Field | Type | Description |
| :--- | :--- | :--- |
| `title` | `String` | Display name. |
| `icon` | `Option<IconSource>` | App logo/favicon. |
| `theme_color` | `Option<[f32; 4]>` | Brand theme color. |
| `display_mode`| `DisplayMode` | App viewing style. |

### `enum IconSource`
- **`Path(String)`**: Load icon from a local file path.
- **`Bytes(Vec<u8>)`**: Load icon from an in-memory buffer.

---

## 🖱️ Interactivity
Types for managing user feedback and hardware pointers.

### `enum CursorIcon`
Standardized mouse cursor shapes supported across Desktop and Web.
