# Terminal Adapter 📟

The **Terminal Adapter** brings the Rupa "Artisan" experience to the command line, transforming reactive logic into ANSI-powered character interfaces.

---

## 🏗️ Technical Architecture
- **IO Backend**: Uses **Crossterm** for raw terminal access and event polling.
- **Grid Engine**: Integrates with the **TUI Orchestrator** to perform "Cell-Rounding" on floating-point layouts.
- **Buffer Rendering**: Uses a double-buffering strategy to minimize flicker and ensure smooth updates.

---

## 🎨 Artisan Capabilities
- **TrueColor Aesthetics**: Full support for 24-bit RGB colors, allowing for beautiful OKLCH-based themes in the terminal.
- **Interactive TUI**: Handles keyboard-heavy navigation, tab cycling, and hotkeys natively.
- **Portable Art**: Runs everywhere SSH can go, providing a consistent UI from local dev to remote servers.

---

## 🗝️ Standard Workflow
1.  **Initialization**: `App::with_mode("My CLI", LayoutMode::Cell)` prepares the Kernel for grid-based manifestation.
2.  **Manifestation**: The adapter maps VNode styles to ANSI escape codes and character boxes.
3.  **Interaction**: Key escape sequences are mapped to Rupa **Focus** and **Navigation** intents.

[Technical Spec: Terminal Blueprint](../../workshop/architectures/terminal-adapter.md)
