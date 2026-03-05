# Forms & Reactive Inputs

Rupaui provides a complete set of semantic form components that map to Bootstrap's layout while leveraging **Signals** for reactive state management and validation.

## 🔡 Input & Textarea

The `Input` component handles text-based user input.

- `.label(String)`: Adds a floating or standard label.
- `.value(Signal<String>)`: Binds the input value to a reactive signal.
- `.state(Signal<FormState>)`: Binds the visual state (`Normal`, `Valid`, `Invalid`).
- `.textarea()`: Converts the input into a multi-line textarea.

```rust
let email = Signal::new(String::new());
let state = Signal::new(FormState::Normal);

Input::new("name@example.com")
    .label("Email Address")
    .value(email)
    .state(state)
```

---

## 🔽 Select

Semantic dropdown for single selections.

```rust
let options = vec![
    ("1".into(), "One".into()),
    ("2".into(), "Two".into())
];
Select::new(options)
```

---

## ✅ Checks & Radios

Individual toggles for boolean or exclusive states.

```rust
let is_agreed = Signal::new(false);

Check::checkbox("Agree to terms")
    .checked(is_agreed);

Check::radio("Exclusive option");
```

---

## 🎚 Range

A slider for numeric input within a specific boundary.

```rust
let volume = Signal::new(50.0);
Range::new(0.0, 100.0).value(volume);
```

---

## 🛡 Validation

Visual feedback is controlled via the `FormState` enum. This is typically used with a `Memo` to calculate validation status dynamically.

```rust
let is_valid = Memo::new(email.clone(), |val| {
    if val.contains("@") { FormState::Valid } else { FormState::Invalid }
});

Input::new("Email").state(is_valid.into_signal());
```

## 🗝 Key Types
- **FormState**: `Normal`, `Valid`, `Invalid`.
- **Role**: Inputs are automatically assigned appropriate ARIA roles (`TextBox`, `Checkbox`, etc.).
