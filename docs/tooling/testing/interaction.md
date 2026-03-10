# Module: User Interaction Simulation (`crates/rupa-test/src/interaction.rs`) 🖱️

This module simulates hardware events against the virtual component tree.

---

## 🏗️ Events
- `tester.click(id)`: Dispatches a `PointerButton::Pressed` event to the component with the matching ID.
- `tester.type_text(id, "text")`: Simulates keyboard input.
- `tester.scroll(id, delta)`: Simulates mouse wheel or touch scrolling.
