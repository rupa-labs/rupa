# Platform Integration: Platform Orchestrator 🏛️

The Platform Orchestrator (Layer 1) is the foundation of every Rupa Framework application. It manages the hardware-specific execution environment and provides a unified "agnostic bridge" for the higher layers of the framework.

---

## 🏗️ Architecture

The Orchestrator follows a **composition-based architecture** to eliminate redundancy across different target platforms.

### Core Components

#### 1. Platform Core (`context.rs`)
The shared state shared across all platforms. It holds the `AppMetadata`, the `Root Component`, the `Scene Graph`, and the `Input Dispatcher`.
- [**Technical Reference: Platform Core**](./platform-orchestrator.md)

#### 2. Platform Runners (`desktop/`, `terminal/`, `web/`, `mobile/`)
Platform-specific "shells" that implement the execution loop (Event Loop).
- [**Desktop Runner (Desktop)**](./desktop-runner.md)
- [**Terminal Runner (CLI/TUI)**](./terminal-runner.md)
- [**Web Runner (WASM/Browser)**](./web-runner.md)
- [**Mobile Runner (Android/iOS)**](./mobile-runner.md)

---

## 🔄 Execution Pipeline

The lifecycle of a Rupa Framework application is standardized across all targets:

1.  **Bootstrap**: The `App` struct initializes global state (Theme, Plugins) and selects the appropriate `PlatformRunner`.
2.  **Environment Setup**: The Runner initializes the hardware abstraction (WGPU, Crossterm, or Canvas) and creates the visual surface (Window or Character Grid).
3.  **The Agnostic Bridge**: The Runner enters its event loop and begins mapping native events into Rupa Framework's `InputEvent`.
4.  **Render Loop**: Higher layers (`SceneCore` and `Renderer`) are invoked to resolve layout and paint the UI tree onto the surface.
5.  **Shutdown**: The Orchestrator ensures all hardware resources are safely released (restoring terminal state or destroying GPU surfaces).

---

## 🛡️ Dependency Inversion (Platform Integration-DI)

To ensure long-term sustainability, Layer 1 strictly follows Dependency Inversion. Every third-party library is isolated behind an "Infra" wrapper:
- **`DesktopInfra`**: Isolates Winit windowing.
- **`TerminalInterface`**: Isolates Crossterm terminal commands.
- **`WebInfra`**: Isolates Web-Sys/JS DOM manipulation.
- **`MobileInfra`**: Isolates native mobile glue logic (JNI/UIKit).
