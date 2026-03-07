# The Theme Engine: DNA Visual 🧬

The Theme Engine manages the global aesthetic identity of a Rupa Framework application. It ensures consistency across all components by centralizing design tokens.

---

## 🧠 Internal Anatomy

### 1. Global Theme State
- **Mechanism:** Stores a thread-local or global `Theme` struct.
- **Tokens:** Contains color palettes, spacing scales, and default rounding values.

### 2. Live Handshake
When a component is created, it performs a **Handshake** with the Theme Engine to retrieve its default variant colors. If the global theme changes (e.g., toggling Dark Mode), the engine notifies the framework to perform a tree-wide redraw.

---

## 🗝️ Public API

- `Theme::current()`: Returns a read-only reference to the active design tokens.
- `Theme::update(|t| ...)`: Mutates the global theme.
- `ColorMode`: Enum for `Light`, `Dark`, and `System` settings.

---

## 🎨 Token Structure
The `Theme` struct organizes tokens into:
- **Colors:** Perceptual OKLCH palette.
- **Scales:** 10-step unified sizing system.
- **Borders:** Default NW/NE/SW/SE rounding.

---

## 🔄 Interaction Flow
- **Theme -> App Bootstrap:** Injected during application startup.
- **Theme -> Component:** Provides default colors and scales for semantic variants during the `render()` phase.
