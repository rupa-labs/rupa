# Primitive: Grid 🏁

The `Grid` primitive is the most advanced layout atom in Rupa Framework. It provides a two-dimensional coordinate system, allowing for the precise placement of elements within rows and columns.

---

## 🧠 Internal Anatomy

### 1. GridLogic
- **Role:** Spatial collection management.
- **Responsibility:** Manages children and (Planned) track definitions for rows and columns.

### 2. GridView
- **Infrastructure:** Composes `ViewCore`.
- **Taffy Mapping:** Sets the Taffy `Display` property to `Grid`. It serves as the bridge between Rupa Framework's styling API and the CSS Grid algorithm.

---

## 🗝️ Public API (Usage)

### Constructor
- `Grid::new()`: Initializes an empty 2D grid container.

### Methods
- `.child(Box<dyn Component>)`: Appends a child into the next available grid cell.

---

## 🎨 Layout Configuration
Used for complex scaffolding like dashboards or data grids.
```rust
Grid::new()
    .style((
        gap(16.0),
        w_full()
    ))
```

---

## 🚀 Performance Note
Unlike a series of nested Flex containers, a single `Grid` can often resolve complex spatial layouts in a single Taffy pass, making it more efficient for dense UIs.
