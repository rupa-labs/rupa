# Interaction & Events Reference 🎯

Rupa Framework abstracts raw hardware signals (Mouse, Keyboard, Touch) into **Semantic Intents**. This ensures that your component's behavior remains consistent across all platforms.

---

## 🏗️ The Unified Input Model

Events are categorized into three primary pillars based on the type of interaction:

### 1. Pointer Events (Positional)
Represent inputs with spatial coordinates.
- **Actions**: `Down`, `Up`, `Move`, `Hover`, `Scroll`.
- **Properties**: `position: Vec2`, `button: Option<PointerButton>`, `modifiers: Modifiers`.

### 2. Focus & Navigation (Logical)
Represent movement through the UI tree.
- **Actions**: `Enter` (Focus Gain), `Leave` (Focus Loss), `Next/Prev` (Tab cycling), `Directional` (Arrow navigation).

### 3. System Events
Physical environment changes like `Resize`, `Quit`, or `WindowFocus`.

---

## ⚡ Semantic Intent Listeners

Artisans should prioritize **Intent Listeners**. These methods trigger based on the user's *goal*, regardless of the input device.

| Listener | Trigger (GUI / Desktop) | Trigger (TUI / Terminal) |
| :--- | :--- | :--- |
| **`.on_submit(f)`** | Primary Click (Left Click) | `Enter` or `Space` key |
| **`.on_cancel(f)`** | Click Outside / Backdrop | `Escape` key |
| **`.on_select(f)`** | Hover / Mouse Enter | `Tab` or `Arrow` focus |

---

## ⌨️ Keyboard & Text API

For components that require direct character input (like `Input` or `TextArea`):

| Method | Description |
| :--- | :--- |
| **`.on_key_down(f)`** | Triggers on any key press. Receives `KeyCode`. |
| **`.on_text_input(f)`**| Triggers when a completed character is produced. |

---

## 🔄 Propagation & Bubbling

1.  **Hit-Testing**: The framework identifies the target component based on Pointer coordinates or current Focus.
2.  **Bubbling**: Events bubble from the target up to the Root. You can call `event.stop_propagation()` to halt this process and prevent parent handlers from triggering.
3.  **Agnostic Routing**: The same `on_submit` logic will work whether the user clicked a pixel on a screen or pressed Enter in a terminal.

### Example: Stopping Propagation
```rust
Button::new("Internal Action")
    .on_click(|event| {
        event.stop_propagation(); // Parent containers won't see this click
        log::info!("Inner button clicked!");
    })
```

*Learn more about the technical dispatcher in the [**Workshop: Input System**](../../workshop/architectures/input-system.md).*
