# ForEach 🔁

A high-performance logic component for rendering reactive collections.

---

## 🏗️ Purpose
`ForEach` takes a `Signal<Vec<T>>` and a template function to produce a list of VNodes. It uses key-based reconciliation to minimize hardware updates.

---

## 🚀 Example
```rust
ForEach::new(users_signal, |user| {
    Text::new(&user.name)
})
```
