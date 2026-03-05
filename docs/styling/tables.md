# Table Utilities

Rupaui provides specialized utilities for managing data-heavy table layouts with precision and semantic clarity.

## 📊 Table Layout

- `.table_layout(TableLayout)`: Choose between `Auto` (content-based) or `Fixed` (width-based).
- `.border_collapse(BorderCollapse)`: Choose between `Collapse` or `Separate`.
- `.table_gap(x, y)`: Set the spacing between table cells (when borders are separate).
- `.caption_side(CaptionSide)`: Position the table caption (`Top` or `Bottom`).

```rust
Style::new()
    .table_layout(TableLayout::Fixed)
    .border_collapse(BorderCollapse::Collapse)
    .w(1000.0)
```

## 🗝 Key Types
- **TableLayout**: `Auto`, `Fixed`.
- **BorderCollapse**: `Collapse`, `Separate`.
- **CaptionSide**: `Top`, `Bottom`.
