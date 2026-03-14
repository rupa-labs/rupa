# Lifecycle Transitions (Enter & Leave) 🚪

Lifecycle transitions define how components animate when they are added to or removed from the Element Tree.

---

## 🏗️ Transition Enter
Defines the animation state from "not mounted" to "mounted".

- **`transition_enter_start(mod)`**: The initial style before the component is visible.
- **`transition_enter_end(mod)`**: The final style once the entry animation completes.

### Example
```rust
Text::new("Hello")
    .transition_enter_start(opacity(0.0).translate_y(10.0))
    .transition_enter_end(opacity(1.0).translate_y(0.0))
```

---

## 🏗️ Transition Leave
Defines the animation state from "mounted" to "destroyed".

- **`transition_leave_start(mod)`**: The style at the moment the unmount is triggered.
- **`transition_leave_end(mod)`**: The final style before the component is physically removed.

### Example
```rust
Modal::new()
    .transition_leave_start(scale(1.0).opacity(1.0))
    .transition_leave_end(scale(0.9).opacity(0.0))
```
