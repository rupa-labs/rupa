# Event System Reference 🎯

Rupa Framework provides a sophisticated event routing system that bridges raw Operating System signals with your UI components. This document details the types of events you can handle and how they propagate.

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

### 1. Capture & Bubble
Input events follow a standard two-phase lifecycle:
- **Capture Phase**: The event travels from the Root down to the target component.
- **Bubble Phase**: The event travels from the target back up to the Root. Most component handlers (`on_click`, `on_hover`) operate during this phase.

### 2. Event Consumption
If any handler calls `.consume()` during propagation, the event is immediately halted and will not reach further components in the pipeline.

### 3. Focused Routing
Keyboard and text input events are sent directly to the component that has requested focus (e.g., an `Input` field), bypassing hit-testing.

---

## 📱 Platform Mapping (The Agnostic Bridge)

Rupa Framework Runners (Desktop, Web, Mobile) perform automated translation:
- **Winit (Desktop)**: Mouse movements and clicks are mapped to `PointerMove` and `PointerButton`.
- **Mobile (Touch)**: Single and multi-touch events are normalized into `Pointer` events by the `rupa-mobile-core` composite assembly.
- **Web (JS)**: DOM events (PointerEvent, KeyboardEvent) are serialized and bridged into the WASM runner.
