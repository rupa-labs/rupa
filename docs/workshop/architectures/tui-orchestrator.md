# TUI Orchestrator Architecture 🛠️

The **TUI Orchestrator** is a Tier 2 Composite that assembles **Console Atoms** into a managed character-grid UI system.

---

## 1. Core Responsibilities

- **Character Grid Mapping**: The "Brain" that decides how to fit floating-point layouts into integer character cells.
- **Terminal Component Library**: High-level logic for terminal-specific elements like `Panel`, `List`, and `Window`.
- **ANSI Styling Mapper**: Translates Rupa's agnostic `Style` DNA into specific ANSI escape sequences.

---

## 2. Technical Context

- **Kernel Integration**: Consumes the **Reconciliation Kernel** to identify TUI-specific updates.
- **Agnostic Logic**: While it targets terminals, the core logic is decoupled from any specific IO backend (like Crossterm).
- **Physical Preparation**: Prepares the "Frame" that the **Terminal Adapter** will eventually draw.
