# Overlay Materials 🌌

Overlays are components that appear on top of the main application content. They manage their own spatial coordinates and Z-stacking priority.

---

## 🏗️ Core Materials

### `Overlay` (The Foundation)
The low-level primitive for all floating elements. It provides the base logic for visibility and absolute positioning.

### `Modal` (Dialog)
A focused dialog box that captures the user's attention. Modals typically block interaction with the background content until they are dismissed.

### `Tooltip` (Contextual Hint)
A small, non-blocking label that appears when hovering or focusing on an element.

---

## 🗝️ API & Usage

### Visibility Control
Overlays are typically controlled via a `Signal<bool>`.

```rust
let is_modal_open = Signal::new(false);

Modal::new()
    .child(Text::new("Are you sure?"))
    .on_click(|_| is_modal_open.set(false))
```

---

## 🎨 Layout & Positioning
By default, overlays use `Position::Absolute`. Their exact location can be controlled using the `.absolute()` and `.z(val)` style utilities.

- **`Modal`**: Usually centered in the viewport.
- **`Tooltip`**: Positioned relative to its trigger component.
- **`Toast`**: Positioned at the corners of the viewport.
