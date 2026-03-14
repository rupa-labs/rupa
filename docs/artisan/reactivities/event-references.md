# Event System Reference 🎯

Rupa Framework provides a sophisticated event routing system that bridges raw Operating System signals with your UI components. This document details the types of events you can handle and how they propagate.

---

## 🖱️ Interaction Types
Agnostic events translated from raw OS signals.

| Event Type | Properties | Description |
| :--- | :--- | :--- |
| `Pointer` | `pos`, `delta`, `button` | Clicks, movements, and scrolling. |
| `Key` | `key`, `pressed`, `modifiers` | Keyboard input and hotkeys. |
| `Focus` | `bool` | Element focus and blur events. |

---

## 🏗️ The `UIEvent` Schema
Every event handler in a component receives a `UIEvent`.

### `struct PointerEvent`
- **`.pos`**: The (x, y) coordinates relative to the component.
- **`.modifiers`**: Access state of `Shift`, `Ctrl`, `Alt`, and `Meta`.
- **`.button`**: Identification of `Primary`, `Secondary`, or `Middle` buttons.

---

## ⚡ Event Handlers
Components store reactive callbacks in the `handlers` field of a `VElement`.

```rust
Button::new("Click Me")
    .on_click(|event| {
        if let UIEvent::Pointer(e) = event {
            println!("Clicked at {:?}", e.pos);
        }
    })
```

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
