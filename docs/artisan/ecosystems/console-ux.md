# Console UX Standards 🎨

The Rupa CLI uses a unified set of standardized components for all terminal outputs. This ensures that every command feels like a coherent part of the Rupa ecosystem.

---

## 🏛️ Semantic Typography

Instead of raw text, we use semantic levels to convey meaning through color and icons:

| Level | Icon | Color | Usage |
| :--- | :--- | :--- | :--- |
| **Plain** | - | White | Standard informational text. |
| **Info** | ℹ | Blue | Contextual details or hints. |
| **Success** | ✔ | Green | Completed tasks or positive results. |
| **Warning** | ⚠ | Yellow | Potential issues that don't stop execution. |
| **Error** | ✖ | Red | Critical failures. |

---

## 🏗️ Structural Components

### 1. Console Box
Used to group related information or highlight primary outputs.
```text
┌──────────────────────────────┐
│        RUPA FRAMEWORK        │
├──────────────────────────────┤
│  CLI Version: 0.1.0          │
└──────────────────────────────┘
```

### 2. Progress Bar
A visual indicator for long-running processes (Build, Update).
```text
Compiling: [████████░░░░] 60%
```

### 3. Horizontal Lines
Subtle separators used to organize output sections without adding clutter.

---

## 🛠️ Implementation

All CLI commands MUST use the `Console` utility provided in `rupa-cli::console`. This ensures that even simple output follows the framework's quality standards.
