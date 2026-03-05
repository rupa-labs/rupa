# Layout, Flexbox & Grid

Rupaui separates layout concerns into three logical sub-modules to maintain a clean and performant architecture.

## 📐 Common Layout (`layout.rs`)

General properties applicable to all semantic components.

- **Display**: `.flex()`, `.grid()`, `.block()`, `.inline()`, `.display(Display)`
- **Positioning**: `.relative()`, `.absolute()`, `.fixed()`, `.top()`, `.right()`, `.bottom()`, `.left()`, `.z()`
- **Visibility**: `.visibility(Visibility)`
- **Sizing**: `.w()`, `.h()`, `.aspect()`

---

## 🤸 Flexbox (`flex.rs`)

One-dimensional layout utilities.

- **Direction**: `.row()`, `.col()`
- **Wrapping**: `.wrap(FlexWrap)`
- **Growth/Shrink**: `.grow(f32)`, `.shrink(f32)`, `.basis(f32)`
- **Alignment**: `.items(AlignItems)`, `.justify(JustifyContent)`
- **Order**: `.order(i32)`

```rust
Style::new()
    .flex().row()
    .justify(JustifyContent::SpaceBetween)
    .items(AlignItems::Center)
```

---

## 🏁 Grid (`grid.rs`)

Two-dimensional layout utilities.

- **Templates**: `.grid_cols(Vec<String>)`, `.grid_rows(Vec<String>)`
- **Spans**: `.col_span(String)`, `.row_span(String)`
- **Flow**: `.auto_flow(GridAutoFlow)`
- **Spacing**: `.gap(f32)`, `.gap_xy(x, y)`

```rust
Style::new()
    .grid()
    .grid_cols(vec!["1fr", "auto", "1fr"])
    .gap(16.0)
```
