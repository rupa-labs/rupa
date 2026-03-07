# Primitive: Div 📦

The `Div` (Division) is the most fundamental structural atom in Rupa Framework. It serves as a generic container for grouping components and applying baseline styles without any pre-defined layout constraints.

---

## 🧠 Internal Anatomy

### 1. DivLogic
- **Role:** Pure hierarchy management.
- **Responsibility:** Maintains an ordered collection of children (`Children<'a>`). It is logic-light, delegating all spatial concerns to the View.

### 2. DivView
- **Role:** Structural manifestation.
- **Infrastructure:** Composes `ViewCore` to manage its link to the Geometric Scene Layer.
- **Taffy Mapping:** By default, it creates a "Block" node in the Taffy tree, acting as a flexible container for its descendants.

---

## 🗝️ Public API (Usage)

### Constructor
- `Div::new()`: Initializes an empty generic container.

### Methods
- `.child(Box<dyn Component>)`: Appends an element to the internal collection.

---

## 🎨 Styling Standard
As a primitive, `Div` is the primary target for atomic visual modifiers:
```rust
Div::new()
    .style((
        bg(Color::Zinc(900)),
        p(16.0),
        rounded(8.0)
    ))
```

---

## ⚡ Interaction
- **Events:** Supports hit-testing but does not implement any default semantic interaction logic.
- **Reactivity:** Marks its `ViewCore` as `dirty` whenever a child is added, ensuring the layout tree is recalculated.
