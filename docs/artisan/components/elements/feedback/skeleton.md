# Skeleton 🦴

Visual placeholders used while content is being fetched.

---

## 🏗️ Purpose
`Skeleton` components improve perceived performance by mimicking the layout of the content that is about to load, preventing layout shifts.

---

## 🚀 Example
```rust
VStack::new()
    .child(Skeleton::new().h(Scale::Md).w(Scale::Xl4))
    .child(Skeleton::new().h(Scale::Sm).w(Scale::Xl2))
```
