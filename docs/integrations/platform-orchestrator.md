# Platform Integration: Platform Orchestrator 🏛️

The Platform Orchestrator (provided by `rupa-engine` and `rupa-mobile`) is the foundation of every Rupa Framework application. It manages the hardware-specific execution environment and provides a unified "agnostic bridge" for the framework's higher-level reactive logic.

---

## 🏗️ Architecture

The Orchestrator follows a **composition-based architecture** to eliminate redundancy across different target platforms.

### Core Components

#### 1. Platform Context (`context.rs`)
The shared state across all platforms. It holds the `AppMetadata`, the `VNode Tree`, the `Scene Graph`, and the `Input Dispatcher`.

#### 2. Platform Runners (`desktop/`, `terminal/`, `web/`, `mobile/`)
Platform-specific "shells" that implement the execution loop (Event Loop).
- [**Desktop Runner (WGPU)**](./desktop-runner.md)
- [**Terminal Runner (TUI)**](./terminal-runner.md)
- [**Web Runner (WASM)**](./web-runner.md)
- [**Mobile Runner (Mobile)**](./mobile-runner.md)

---

## 🔄 Execution Pipeline

The lifecycle of a Rupa Framework application is standardized across all targets:

1.  **Bootstrap**: The `App` struct initializes global state (Theme, Plugins) and selects the appropriate `PlatformRunner`.
2.  **Environment Setup**: The Runner initializes the hardware abstraction (WGPU, Crossterm, or Canvas) and creates the visual surface (Window, Mobile Activity, or Character Grid).
3.  **The Agnostic Bridge**: The Runner enters its event loop and begins mapping native events into Rupa Framework's agnostic `InputEvent`.
4.  **Reactive Render Loop**:
    *   **Build**: Components generate new VNode sub-trees.
    *   **Diff**: The core reconciliation engine identifies changes.
    *   **Patch**: The Renderer executes targeted updates (Taffy layout & GPU/TUI paint).
5.  **Shutdown**: The Orchestrator ensures all hardware resources are safely released (restoring terminal state or destroying GPU surfaces).

---

## 🛡️ Dependency Inversion (Platform-DI)

To ensure long-term sustainability, the Platform layer strictly follows Dependency Inversion. Every third-party library is isolated behind an "Infra" wrapper:
- **`DesktopInfra`**: Isolates Winit windowing and desktop-specific OS calls.
- **`TerminalInterface`**: Isolates Crossterm terminal commands.
- **`WebInfra`**: Isolates Web-Sys/JS DOM manipulation.
- **`MobileInfra`**: Isolates native mobile glue logic (JNI for Android, Swift/Obj-C for iOS).
