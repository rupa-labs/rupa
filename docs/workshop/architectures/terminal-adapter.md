# Terminal Adapter Architecture 📟

The **Terminal Adapter** is a Tier 3 Showroom that provides the physical implementation for running Rupa applications in a terminal.

---

## 1. Core Responsibilities

- **Hardware Execution**: Uses **Crossterm** to physically manipulate the terminal screen and capture raw input.
- **Terminal Runner**: Provides the `TerminalRunner` that manages the main event loop for TUI applications.
- **Input Intent Mapping**: Translates raw terminal key events into Rupa's semantic **Focus** and **Navigation** intents.

---

## 2. Technical Composition

- **Fulfills the Renderer Port**: Implements drawing logic using character buffers.
- **Consumes TUI Orchestrator**: Uses the high-level TUI logic to manage the terminal grid.
- **Showroom Identity**: The entry point for Artisans building terminal-based masterpieces.
