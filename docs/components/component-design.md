# Architectural Guide: Semantic Component Design 🎨

Semantic Components are the "Artisan Library" of the Rupa Framework. They provide high-level, meaningful UI elements that users interact with. Unlike Primitives, Semantic Components carry intent, accessibility roles, and internal logic.

---

## 🏗️ The Artisan Philosophy

1.  **Meaningful Structure:** Every component represents a specific UI pattern (e.g., a `Navbar` is a navigational landmark).
2.  **Logic-Heavy:** Components at this level handle reactive states, data validation, and event orchestration.
3.  **VNode Architecture:** They describe their visual structure by producing a `VNode` tree in their `render()` method.
4.  **Agnostic Interaction:** Components remain decoupled from the rendering backend, enabling them to run on Desktop, Mobile, and Web.

---

## 🛠️ Implementation Standard

To build a new Artisan component in the Rupa Framework:

1.  **Reactive State:** Use `Signals` and `Memos` for all internal data that affects the UI.
2.  **VNode Rendering:** Implement the `Component` trait. The `render()` method should return a tree of VNodes (often built by composing Primitives like `Div` and `Flex`).
3.  **Stylable Trait:** Implement `Stylable` to allow users to customize the component using functional modifiers.
4.  **Semantic API:** Provide ergonomic methods (chaining) that speak the language of the component (e.g., `.variant()`, `.loading()`, `.on_click()`).

---

## 🧩 The Artisan Categories

| Category | Components |
| :--- | :--- |
| **Actions** | `Button`, `ButtonGroup` |
| **Forms** | `Input`, `Select`, `Checkbox`, `Radio`, `Switch`, `Label` |
| **Feedback** | `Alert`, `Badge`, `Progress`, `Spinner`, `Skeleton` |
| **Navigation** | `Navbar`, `Tabs`, `Breadcrumb` |
| **Overlays** | `Modal`, `Tooltip` |
| **Content** | `Card`, `Table`, `Accordion` |

---

## 🔄 Interaction Flow

- **Components -> Primitives:** Artisan components are built by composing Primitives like `Div` and `Flex`.
- **Components -> Reactivity:** They are the primary consumers of the `rupa-signals` reactivity engine.
- **Components -> Styling:** They utilize design tokens from `rupa-styling` to ensure aesthetic consistency.
