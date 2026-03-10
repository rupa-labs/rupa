# Module: Reactive Validation (`crates/rupa-forms/src/validation.rs`) ✅

This module implements the core validation logic for form inputs.

---

## 🏗️ Features
- **Built-in Rules**: Email, Required, Length, Regex, Numeric Range.
- **Custom Validators**: Ability to provide closures that return `Result<(), String>`.
- **Async Validation**: Support for server-side checks (e.g., "username already taken").
