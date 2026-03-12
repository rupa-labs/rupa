# `rupa-forms` 📝

**The Form & Validation Engine.** Reactive form objects and Laravel-inspired validation rules.

## 🛠️ Key Features

- **`Form` (Trait & Macro)**: Encapsulate form logic into clean objects.
- **`Field<T>`**: Reactive state for individual inputs (Value, Error, Dirty).
- **`Validator`**: Rule-chaining engine with `nullable` and `sometimes` support.
- **`Rule`**: Port for custom validation logic.

## 🚀 Usage

```rust
use rupa_forms::{Form, Field, Validator, std_rules::*};

#[derive(Form)]
struct ContactForm {
    name: Field<String>,
    email: Field<String>,
}

// 1. Setup with validation
let form = ContactForm {
    name: Field::new("".into(), Validator::new().rule(Required)),
    email: Field::new("".into(), Validator::new().rule(Required).rule(Email)),
};

// 2. Validate all
if form.validate() {
    submit(form.data());
}
```
