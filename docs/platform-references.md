# Platform & Entry-Point Reference 🚀

This document details the standardized entry-points for every Rupa Framework target platform. Rupa Framework follows a **Target-Driven Execution** model, allowing the same UI logic to be deployed across Desktop, Terminal, Web, and Mobile environments.

---

## 🏛️ Application Entry-Points
The high-level methods used to start the Rupa Framework event loop on various targets.

### `struct App`
The primary orchestrator for bootstrapping and running your application.

| Method | Platform | Description | Example |
| :--- | :--- | :--- | :--- |
| **`.run()`** | Desktop | Starts the OS window and GPU-accelerated loop. | `app.run()` |
| **`.run_terminal()`**| Terminal| Initializes raw mode and the character-grid loop. | `app.run_terminal()` |
| **`.run_web(id)`** | Web | Hooks into a specific browser `<canvas>` element. | `app.run_web("canvas")` |
| **`.run_mobile()`** | Mobile | Manages native mobile lifecycles (Suspend/Resume). | `app.run_mobile()` |

---

## 🛠️ Target Platform Matrix
Technical requirements and internal infrastructure for each supported target.

| Platform | Feature | Internal Runner | Graphics Engine |
| :--- | :--- | :--- | :--- |
| **Desktop** | `desktop` | `DesktopRunner` | GUI (WGPU) |
| **Terminal**| `terminal`| `TerminalRunner`| TUI (Crossterm) |
| **Web** | `web` | `WebRunner` | GUI (Canvas/WGPU) |
| **Mobile** | `mobile` | `MobileRunner` | GUI (WGPU) |

---

## ⚙️ Metadata Configuration
Configure your application's identity and appearance using the `App` manifest API.

### Identity & Branding
```rust
App::new("Artisan Pro")
    .author("Reasnov")
    .version("1.2.0")
    .icon(IconSource::Path("assets/logo.png".into()))
```

### Display & Theming (PWA / Manifest)
These properties are synchronized with the browser meta tags or mobile system bars.
```rust
App::new("Demo")
    .theme_color([0.1, 0.1, 0.1, 1.0]) // Dark theme color
    .display_mode(DisplayMode::Standalone) // PWA Standalone
```

---

## 🏗️ Platform Runners (Platform Integration)
The low-level shells responsible for hardware abstraction and event mapping.

### `trait PlatformRunner`
The common contract implemented by all target execution shells.

| Method | Description |
| :--- | :--- |
| **`.run()`** | Starts the platform-specific event loop and handles lifecycle events. |

---

## 🔄 Interaction Flow (Agnostic Bridge)

Regardless of the entry-point, every Rupa Framework application follows a unified execution flow:

1.  **Bootstrap**: `App` initializes global themes, signals, and plugin registries.
2.  **Environment Sync**: The Runner synchronizes `AppMetadata` with the host OS (Window title, Browser title, etc).
3.  **Event Mapping**: The Runner translates native events (Winit, JS, Crossterm) into agnostik `InputEvent` types.
4.  **Render Loop**: The Runner invokes the `SceneCore` and `Renderer` to resolve and paint the UI tree.

---

## 🛡️ Target-Specific Implementation Detail

### Desktop (Windows, macOS, Linux)
Utilizes **Winit** for window management and **WGPU** for high-performance rendering. Requires the `desktop` feature (enabled by default).

### Terminal (CLI)
Utilizes **Crossterm** for character grid management. Requires the `terminal` feature. Focuses on minimal resource usage and fast CLI interactions.

### Web (WASM)
Compiles to **WebAssembly** and targets the browser's Canvas API. Uses `wasm-bindgen` and `web-sys` for DOM interaction via the `WebInfra` wrapper.

### Mobile (Android, iOS)
Handles the complex **Suspend/Resume** lifecycle. Reclaims GPU resources automatically when the application is backgrounded via the `MobileRunner`.
