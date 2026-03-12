# `rupa-forms` 📝

**The Interaction Port.** This crate provides **Atoms** for reactive form management and data validation. It uses a schema-driven approach to keep form logic decoupled from UI components.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Chained validation rules ensure data integrity before submission.
    - **Sustain (S2)**: `Field<T>` atoms encapsulate value, error, and dirty state into a single reactive unit.
    - **Scalable (S3)**: Schema-based validation supports complex, nested form structures.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Manager`** | Form Orchestrator. | Tracks multiple fields and overall validation status. |
| **`Field<T>`** | Reactive Input. | Value, Error (String), and Dirty (bool) signals. |
| **`Validator`** | Rule Engine. | Laravel-inspired rule chaining (Required, Email, Min, Max). |
| **`Scope`** | Context Scope. | Provides a localized container for form state. |
| **`MockForm`** | Testing Backend. | Simulates form submission and captures data for tests. |

## 🚀 Usage

```rust
use rupa_forms::{Manager, Field, Validator, std_rules::*};

// 1. Define reactive fields
let email = Field::new("".into(), Validator::new().rule(Required).rule(Email));

// 2. Update value from UI
email.value.set("artisan@rupa.rs".into());

// 3. Validate
if email.validate() {
    println!("Valid input!");
} else {
    println!("Error: {}", email.error.get().unwrap());
}
```

## 🧪 Testing & Reliability
- **TDD Support**: `MockForm` allows tests to verify submission logic without a physical frontend.
- **Rule Verification**: Built-in rules are verified for accuracy (e.g., Regex patterns, Numeric bounds).
- **Decoupled**: The form logic doesn't know about `input` or `textarea` elements; it only manages reactive state.
