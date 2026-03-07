# Layer 7 Architecture: Semantic Component Design 🎨

Layer 7 is the "Artisan Library" of Rupa Framework. It provides high-level, meaningful UI elements that users interact with. Unlike Primitives (L6), Semantic Components carry intent, accessibility roles, and complex internal logic.

---

## 🏗️ The Artisan Philosophy

1.  **Meaningful Structure:** Every component represents a specific UI pattern (e.g., a `Navbar` isn't just a box; it's a navigational landmark).
2.  **Logic-Heavy:** Components at this layer handle complex states, data validation, and event orchestration.
3.  **Standardized Bridge:** They strictly follow the **Logic & View** pattern, delegating visual infrastructure to `ViewCore`.
4.  **Accessible by Design:** Every component must implement appropriate ARIA-like roles and accessibility metadata using the AccessKit bridge (L1).

---

## 🛠️ Implementation Standard

To build a new Artisan component in Layer 7:

1.  **Define Logic:** Create a `Logic` struct containing all reactive `Signals` and event handlers. It must remain rendering-agnostic.
2.  **Define View:** Create a `View` struct that composes `ViewCore`. It handles how the component translates its logic state into paint commands.
3.  **Create the Bridge:** Implement the `Component` and `Stylable` traits on the main struct.
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

## 🔄 Interaction
- **L7 -> L6:** Artisan components are built by composing Primitives like `Div` and `Flex`.
- **L7 -> L4:** They are the primary consumers of the Reactivity Layer.
- **L7 -> L9:** They utilize DNA Visual tokens to ensure aesthetic consistency.
