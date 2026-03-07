# Architecture: Debug Mode & Error Handling 🛠️

Debug Mode is an essential pillar of Rupaui's developer experience (DX). It provides visibility into the framework's internals, facilitates rapid troubleshooting, and ensures that errors are handled gracefully without compromising the user experience.

---

## 🔍 Debug Mode Overview

Debug Mode can be toggled during application construction. When enabled, it activates several subsystems designed for development.

### 1. Visual Debugging (Wireframes)
- **Bounding Boxes:** Every component's layout bounds are outlined.
- **Hit-Test Visualization:** Shows where the pointer is interacting and the path of the bubbled event.
- **Performance Overlay:** Real-time display of FPS, frame time, and layout calculation time.

### 2. Enhanced Diagnostic Logging
- Logs every `InputEvent` dispatched.
- Detailed trace of `Signal` mutations and reactive updates.
- Warnings for inefficient layouts (e.g., deeply nested containers).

---

## 🛡️ Exception & Error Handling

Rupaui follows a **Fail-Safe** philosophy. Errors are categorized and handled based on their severity.

### 1. `RupauiError` System
The framework uses a centralized error enum to handle common failures:
- **`LayoutError`:** Failures in Taffy tree synchronization.
- **`RendererError`:** GPU or Surface related issues.
- **`PlatformError`:** Windowing or Event Loop failures.

### 2. Error Boundaries
Components can be wrapped in a logical boundary that prevents a single component's failure from crashing the entire application. Instead, a "Fallback View" is rendered.

### 3. Graceful Panics
In Debug Mode, panics provide a detailed stack trace and state snapshot. In Production, panics are caught and reported via telemetry (if enabled) while attempting to keep the event loop alive.

---

## 🗝️ Public API

```rust
App::new("My App")
    .debug(true) // Activates Debug Mode
    .on_error(|err| {
        // Custom global error handler
        eprintln!("Rupaui Error: {:?}", err);
    })
```

---

## 🔄 Internal Implementation
- `src/platform/app.rs`: Manages the debug flag and global error handler.
- `src/renderer/core.rs`: Logic for drawing debug overlays (bounding boxes).
- `src/support/error.rs`: Centralized Error definitions.
