# Module: Advanced Form Engine (`crates/rupa-forms`) 📝

The `rupa-forms` crate provides a robust system for managing complex form states, validation rules, and input lifecycles.

---

## 1. Core Philosophy: Reactive Validation
Form errors and states are treated as first-class signals that update the UI in real-time as the user types.

## 2. Module Structure (1:1 Mapping)
- `validation.rs`: Core logic for data verification (Regex, Range, Custom).
- `form_state.rs`: The `Form` struct that tracks `dirty`, `touched`, and `valid` flags.
- `schema.rs`: Declarative schema definitions for complex multi-field validation.
