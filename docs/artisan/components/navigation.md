# Components: Navigation 🧭

Navigation components provide the high-level spatial landmarks of a Rupa Framework application. They are designed to manage large-scale UI transitions and branding zones.

---

## 🧠 Internal Anatomy

### 1. The Zoning Logic
Navigation elements often divide space into "Zones" (`start`, `center`, `end`). The logic handles which components belong to which zone and manages their collective accessibility roles.

### 2. The Switcher View
For components like `Tabs`, the View is responsible for **Conditional Layout**. It only requests a Taffy node for the active child, saving resources by not calculating or painting hidden tabs.

---

## 🗝️ Standard Components

### `struct Navbar`
- **Anatomy:** Logic manages three list of children. View arranges them horizontally.
- **API:** `.start()`, `.center()`, `.end()`.

### `struct Tabs`
- **Anatomy:** Logic tracks a `Signal<usize>`. View reacts by rendering only the indexed content.
- **API:** Takes a `Vec<Tab>` and a `Signal`.

---

## 🎨 Common Utilities
Navigation components are typically styled with global layout modifiers:
- `.style(flex())`
- `.style(gap(16.0))`
