# Agnostic UI Architecture 🎨

**Agnostic UI** is the Tier 2 semantic component library of the Rupa Framework. It provides high-level, meaningful UI elements that Artisans use to build applications.

---

## 1. Core Principles

- **Semantic Meaning**: Components are named for their purpose (e.g., `Button`, `VStack`, `Card`) rather than their appearance.
- **Platform Agnostic**: A `rupa-ui` component does not know if it will be rendered on a GPU, a terminal, or a web browser.
- **DNA Consuming**: Components utilize the `rupa-vnode` DNA and `rupa-signals` reactivity.

---

## 2. Component Categories

- **Layout Primitives**: `VStack`, `HStack`, `Grid`, `Spacer`.
- **Content Elements**: `Text`, `Image`, `Icon`.
- **Interactive Controls**: `Button`, `Input`, `Checkbox`, `Switch`.
- **Complex Compositions**: `Navbar`, `Sidebar`, `Modal`, `Toast`.

---

## 3. Technical Integration

- **Styling**: Uses the `Style` model to declare visual intent.
- **Reactivity**: Automatically re-renders when internal or provided Signals change.
- **Hooks**: Implements the semantic `Component` trait hooks (`mount`, `render`, `updated`, etc.).
