# Module: Form State Machine (`crates/rupa-forms/src/form_state.rs`) 🔄

This module manages the lifecycle and status of a complete form.

---

## 🧩 `struct Form`
A reactive object that aggregates the state of multiple fields.

- **Flags**: `is_dirty`, `is_touched`, `is_valid`, `is_submitting`.
- **Reset**: Reverts all fields to their initial values.
- **Submission**: Orchestrates the serialization of form data into a single object.
