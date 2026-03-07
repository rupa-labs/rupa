# Architectural Guide: Primitive Design 🧱

Primitives are the foundational building blocks of the Rupa Framework ecosystem. They provide the fundamental structural containers that every high-level component and application view uses for layout.

---

## 🏗️ The Primitive Philosophy

1.  **No Semantic Meaning:** Unlike a `Button` or `Navbar`, a `Div` has no inherent purpose other than to hold other elements.
2.  **Logic-Light:** Primitives focus on **Children Management**. They should never contain business logic or complex internal state.
3.  **Layout-Centric:** Their primary responsibility is to translate styling properties (from `rupa-styling`) into the geometric scene nodes consumed by the rendering engine.
4.  **VNode Mapping:** Every primitive renders as a `VNode::Element` with specific layout-oriented styles.

---

## 🛠️ Implementation Standard

To build a new primitive in the Rupa Framework:

1.  **VNode Rendering:** Implement the `Component` trait and return a `VNode::Element`.
2.  **Composition API:** Provide a clean `.child()` method to allow for intuitive nesting.
3.  **Stylable Trait:** Ensure the primitive implements `Stylable` so users can apply functional modifiers (e.g., `p(16.0)`, `bg_primary()`).

---

## 🧩 The Primitive Stack

| Primitive | Purpose | Internal Mapping |
| :--- | :--- | :--- |
| **Div** | Basic block/container. | Default Flex container. |
| **Flex** | One-dimensional layout. | `Display::Flex` with direction. |
| **Grid** | Two-dimensional layout. | `Display::Grid`. |
| **Overlay** | Floating/Absolute layers. | `Position::Absolute` with Z-index. |

---

## 🔄 Interaction Flow

- **Styling -> Primitives:** Primitives are the primary consumers of functional UI utilities (`gap`, `flex`, `p`).
- **Primitives -> Rendering Engine:** They generate the majority of nodes in the scene graph used for Taffy layout calculations.
- **Primitives -> Semantic Components:** They serve as the structural skeleton for every semantic element (e.g., a `Button` is often a `Div` or `Flex` with interaction logic).
