# Event System Reference 🎯

Rupaui provides a sophisticated event routing system that bridges raw Operating System signals with your UI components. This document details the types of events you can handle and how they propagate.

---

## 🖱️ Pointer Events
Triggered by mouse, touch, or stylus interactions.

| Event Type | Logic | Description |
| :--- | :--- | :--- |
| `PointerMove` | Bubble | Emitted when the cursor changes position. |
| `PointerButton` | Bubble | Emitted on click or release (Primary, Secondary, etc.). |
| `PointerScroll` | Bubble | Emitted when the user scrolls via wheel or trackpad. |

---

## ⌨️ Keyboard & Text Events
Focused interactions for productivity and data entry.

| Event Type | Logic | Description |
| :--- | :--- | :--- |
| `Key` | Priority | Raw key presses (Enter, Escape, Arrows, etc.). |
| `Ime` | Priority | Support for international and complex characters via `on_text`. |

---

## 🏗️ The `UIEvent` Object
Every event hook in a component receives a mutable reference to a `UIEvent`.

| Property/Method | Purpose |
| :--- | :--- |
| `.consume()` | Stops the event from bubbling up to parent components. |
| `.local_pos` | The (x, y) coordinates relative to the component's top-left. |
| `.modifiers` | Access the state of Shift, Ctrl, Alt, and Logo keys. |
| `.request_focus()` | Requests the framework to route future keyboard events here. |
| `.capture_pointer()` | "Grabs" the cursor for interactions like dragging. |

---

## 🔄 Propagation Logic

### 1. Bubbling (Bottom-Up)
Most input events start at the **deepest** component under the cursor and move up to the root. If a child calls `.consume()`, the parent will not receive the event.

### 2. Broadcasting (Top-Down)
System events like **Resize** are sent to every component in the tree simultaneously to ensure responsive updates.

### 3. Focused Routing
Keyboard events are sent directly to the component that has requested focus, bypassing hit-testing until the focus is lost or cleared.
