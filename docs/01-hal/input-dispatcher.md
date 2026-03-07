# Module: Input Dispatcher (`src/platform/dispatcher.rs`) 🎯

The Input Dispatcher is the central intelligence for routing input from the Operating System to the correct UI component. It manages hit-testing, event bubbling, and advanced interaction states.

---

## 🛠️ Key Concepts

### 1. Event Bubbling (Bottom-Up)
Events in Rupaui propagate from the **leaf** node (child) up to the **root** (parent). 
- **Mechanism:** The dispatcher finds the target path and iterates from the last element to the first (`.iter().rev()`).
- **Consumption:** Any component can call `event.consume()` to stop the bubbling and prevent parent elements from receiving the event.

### 2. Pointer Capture (Sticky Interaction)
Enables a component to "grab" the mouse cursor, ensuring it receives all future movement events even if the cursor leaves its physical bounds.
- **Use Case:** Scrollbars, Sliders, and Drag-and-Drop operations.
- **API:** Managed through `event.capture_pointer()` and `event.release_pointer()`.

### 3. Keyboard Focus Manager
Keyboard events are routed to a "focused" element rather than just the hovered one.
- **Priority:** The dispatcher first attempts to send keys to the `focused_id`. If no focus exists, it falls back to the component under the cursor.
- **API:** Managed through `event.request_focus()` and `event.blur()`.

### 4. System Broadcasting (Resize)
Unlike input events that bubble up, system events like **Resize** are broadcasted down.
- **Mechanism:** The dispatcher recursively traverses the tree to call `on_resize` on every component.

---

## 🗝️ Input Routing Logic

| Event Type | Routing Strategy |
| :--- | :--- |
| **Pointer Move** | If captured: Direct to captor. Else: Bubbles from child under cursor. |
| **Pointer Button** | Bubbles from child under cursor. Handles Pressed and Released states. |
| **Pointer Scroll** | Bubbles from child under cursor. |
| **Keyboard Key** | Priority to Focused element. Fallback to hovered element. |
| **IME (Text)** | Direct to Focused element via `on_text`. Supports complex characters. |
| **Resize** | Recursive Broadcast to all components via `on_resize`. |

---

## 🏗️ UIEvent Structure
The `UIEvent` passed to components contains:
- `consumed`: Boolean flag for event flow.
- `local_pos`: Pointer position relative to the component's top-left.
- `modifiers`: Current state of Shift, Ctrl, Alt, and Logo keys.
- `button` & `button_state`: Context for mouse clicks (Primary, Secondary, etc).
## 🌟 Advanced Interaction Support

### Gestures & Multi-Touch
The dispatcher is being extended to support complex touch interactions, including:
- **Pinch-to-Zoom** and **Rotation** gestures.
- **Swipe** detection for mobile and tablet navigation.

### Integrated Navigation
Rupaui provides automated focus management for accessibility and productivity:
- **Tab Cycle:** Automatic navigation between interactive components using `Tab` and `Shift+Tab`.
- **Keyboard Fallbacks:** Smart routing of events to hovered elements when no explicit focus is set.

### OS-Level Bridge
Input events are further enhanced with native OS capabilities:
- **Clipboard Management:** Direct access to get and set system clipboard data from component event hooks.
- **Data Ingestion:** Support for native Drag-and-Drop data payloads.


