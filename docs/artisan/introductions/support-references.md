# Support & Infrastructure Reference 🛠️

This document details the built-in support systems that power Rupa Framework's core logic, from mathematical utilities to reactive state management and accessibility.

---

## 📐 Math & Geometry
Foundational types for spatial calculations and vector manipulation.
- **Internal Reference:** `crates/rupa-support/src/vector.rs`

### `struct Vec2`
A high-precision 2D vector used for layouts, SVG paths, and pointer coordinates.

| Method | Description | Example |
| :--- | :--- | :--- |
| `Vec2::new(x, y)` | Creates a new vector. | `Vec2::new(10.0, 20.0)` |
| `v.length()` | Returns the magnitude of the vector. | `v.length()` |
| `v.normalize()` | Returns a unit vector in the same direction. | `v.normalize()` |

---

## 🆔 ID Generation
Universal unique identification system for VNodes and reactive components.
- **Internal Reference:** `crates/rupa-base/src/id.rs`

### `struct Id`
A lightweight, unique 64-bit identifier.
- **Generation**: Created via `Id::next()`.
- **String Conversion**: Use `.to_string()` for "rupa-X" format.

---

## 🎨 Color System
High-precision color management supporting perceptual uniformity.
- **Internal Reference:** `crates/rupa-base/src/color.rs`

### `struct Color`
The standard RGBA color representation.

| Method | Description | Example |
| :--- | :--- | :--- |
| `Color::rgba(r, g, b, a)` | Creates a linear RGBA color. | `Color::rgba(1.0, 0.0, 0.0, 1.0)` |
| `Color::oklch(l, c, h, a)` | Creates a color using OKLCH. | `Color::oklch(0.8, 0.2, 200.0, 1.0)` |
| `Color::hex(0xRRGGBB)` | Creates a color from Hex. | `Color::hex(0xFF0000)` |

---

## 📐 Math & Geometry
Foundational types for spatial calculations.
- **Internal Reference:** `crates/rupa-base/src/vector.rs` & `rect.rs`

### `struct Vec2`
A 2D vector for layouts and pointer coordinates.

### `struct Rect`
A rectangle representing layout bounds.
- **Fields**: `origin: Vec2`, `size: Vec2`.
- **Methods**: `contains(point)`, `intersects(rect)`, `inset(amount)`.

Fine-grained reactivity system for efficient UI updates.
- **Internal Reference:** `crates/rupa-signals/src/lib.rs`

### `struct Signal<T>`
A thread-safe reactive primitive that notifies the framework to redraw when its value changes.

| Method | Description | Example |
| :--- | :--- | :--- |
| `Signal::new(val)` | Creates a new signal. | `let count = Signal::new(0);` |
| `.get()` | Returns a clone of the current value. | `count.get()` |
| `.set(val)` | Overwrites the value and triggers redraw. | `count.set(10)` |

---

## 📥 Job Queues (Tasks)
Reactive asynchronous task management.
- **Internal Reference:** `crates/rupa-queue/src/lib.rs`
- **Detailed Guide:** [Job Queues](./ecosystems/job-queues.md)

### `struct AsyncQueue`
Manages background tasks with concurrency control.

| Method | Description |
| :--- | :--- |
| `AsyncQueue::new(concurrency)` | Creates a new queue. |
| `.push(task)` | Adds a task to the queue. |
| `.tasks()` | Returns a reactive list of tasks. |

---

## 💾 Persistence (Store)
Bridge between the reactive graph and permanent storage.
- **Internal Reference:** `crates/rupa-store/src/signal.rs`

### `struct PersistentSignal<T>`
A specialized signal that automatically syncs its value to a storage backend.

| Method | Description | Example |
| :--- | :--- | :--- |
| `PersistentSignal::new(key, val)` | Creates a signal linked to a storage key. | `let theme = PersistentSignal::new("theme", Mode::Dark);` |

---

## 🌐 Network (IO)
Reactive primitives for asynchronous I/O operations.
- **Internal Reference:** `crates/rupa-net/src/resource.rs`

### `struct Resource<T>`
A reactive state container for managing the lifecycle of an async network request.

| State | Description |
| :--- | :--- |
| `Loading` | The request is currently in flight. |
| `Ready(T)` | Data has been successfully fetched and parsed. |
| `Error(E)` | The request failed with a network or parsing error. |

---

## 🛡️ Debugging & Error Handling
Robust systems for framework diagnostics and graceful failure recovery.
- **Internal Reference:** `crates/rupa-support/src/error.rs`

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
