# Theme Orchestrator Architecture 🎨

The **Theme Orchestrator** is a shared system that manages global aesthetic presets and perceptual color scaling.

---

## 1. Core Principles

- **OKLCH Color Space**: Uses perceptually uniform color math to ensure consistent contrast and vibrancy across themes.
- **Adaptive Modes**: Built-in support for Light, Dark, and High-Contrast modes.
- **DNA Tokens**: Provides a set of foundational visual tokens (Spacing, Sizing, Colors) that Artisans use to maintain consistency.

---

## 2. Technical Context

- **Reactive Themes**: The current theme is exposed as a Signal, allowing the entire UI to adapt instantly to mode changes.
- **Injection**: Themes are injected into the component tree using the **Context Orchestrator**.
