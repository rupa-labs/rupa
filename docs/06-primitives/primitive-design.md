# Layer 6 Architecture: Primitive Design Guide 🧱

Primitives are the "Atoms" of the Rupaui ecosystem. They provide the fundamental structural containers that every other high-level component (L7) and application view (L8) uses for layout.

---

## 🏗️ The Primitive Philosophy

1.  **No Semantic Meaning:** Unlike a `Button` or `Navbar`, a `Div` has no inherent purpose other than to hold other things.
2.  **Logic-Light:** Primitives follow the **Logic & View** pattern, but their Logic is usually limited to **Children Management**. They should never contain business logic or complex internal states.
3.  **Layout-Centric:** Their primary responsibility is to translate the properties from Layer 9 (UI Utilities) into the Geometric Scene Layer (L3).
4.  **Infinite Nesting:** Primitives are designed to be nested deeply without performance degradation, leveraging the `ViewCore` for efficient updates.

---

## 🛠️ Implementation Standard

To build a new primitive in Layer 6, you must:

1.  **Compose `ViewCore`:** Use the standardized infrastructure for styling and node management.
2.  **Expose `Children`:** Provide a clean `.child()` API for composition.
3.  **Transparent Utilities:** Ensure the primitive implements `Stylable` so users can apply atomic modifiers from Layer 9.

---

## 🧩 The Primitive Stack

| Primitive | Purpose | Taffy Mapping |
| :--- | :--- | :--- |
| **Div** | Basic block/container. | `Display::Block` (or default Flex) |
| **Flex** | One-dimensional layout. | `Display::Flex` |
| **Grid** | Two-dimensional layout. | `Display::Grid` |
| **Overlay** | Floating/Absolute layers. | `Position::Absolute` |

---

## 🔄 Interaction
- **L6 -> L9:** Primitives are the most frequent consumers of UI Utilities (`gap`, `flex`, `p`).
- **L6 -> L3:** They generate the vast majority of nodes in the Taffy tree.
- **L6 -> L7:** They serve as the structural skeleton for every semantic element.
