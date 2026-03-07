# Component: Button 🔘

The `Button` is the primary interactive element in Rupa Framework. It demonstrates the power of the **VNode Architecture**, separating high-level interaction state from platform-specific rendering.

---

## 🏗️ Architecture

### 1. State Management
The Button tracks its internal interactive state via reactive signals:
- **`disabled`**: If true, interaction is blocked and visual state is muted.
- **`is_loading`**: If true, interaction is blocked and a loading indicator is typically rendered.

### 2. VNode Rendering
The `Button` implements the `Component` trait. Its `render()` method translates current state and styles into a `VNode::Element`:
- **Tag**: Produces a `button` tag.
- **Style Mapping**: Automatically applies variants (Primary, Danger) and sizing into the `Style` object.
- **Children**: Renders the label as a `VNode::Text` (or custom children if provided).

---

## 🗝️ API & Usage

### Constructor
- `Button::new(label)`: Initializes the component with a string label.

### Semantic Modifiers
| Method | Description |
| :--- | :--- |
| `.variant(v)` | Sets visual intent (`Primary`, `Danger`, etc). |
| `.size(s)` | Scales the button according to DNA Visual standards. |
| `.on_click(f)` | Attaches a closure to be executed on press. |

---

## 🎨 Styling
Since it implements `Stylable`, users can override any part of the button's appearance:
```rust
Button::new("Save")
    .style((bg_primary(), rounded(4.0), p(12.0)))
```

---

## 🌳 VNode Representation
When `render()` is called, a `Button` produces:
```rust
VNode::Element(VElement {
    tag: "button",
    style: /* computed style based on variant/size */,
    attributes: /* includes "disabled" if applicable */,
    children: vec![VNode::Text("Label".into())],
    key: Some(id),
})
```
