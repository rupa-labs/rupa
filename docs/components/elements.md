# UI Elements Reference

Rupaui features a library of semantic components, mapping modern design patterns to high-performance Rust logic.

## 🔘 Action & Input
- `Button`, `ButtonGroup`: Standard action triggers.
- `Input`, `Select`, `Checkbox`, `Radio`, `Switch`, `Label`: Reactive form controls.

## 📢 Feedback & Status
- `Alert`, `Badge`: Contextual notifications and labels.
- `Progress`: Progress bar driven by a `Signal<f32>`.
- `Spinner`: Visual loading indicator.
- `Skeleton`: Skeleton screens for asynchronous loading.

---

## 🧭 Navigation
- `Navbar`: Top-level branding and navigation area.
- `Tabs`: Tabbed interface for content switching.
- `Breadcrumb`: Hierarchical path tracking.

---

## 📦 Content & Disclosure
- `Card`: Grouped content container.
- `Accordion`: Collapsible content groups.
- `Table`: Data grid component.

---

## 🎭 Overlays
- `Modal`: Centered overlay blocking interactions.
- `Tooltip`: Small floating text description.

```rust
let is_modal_open = Signal::new(false);

Modal::new(is_modal_open.clone())
    // Modals can have headers, footers and children
```

## 🗝 Implementation Standards
- **Signal-First**: Visibility (`Modal`) and values (`Progress`, `Input`) are bound to reactive Signals.
- **Semantic Structure**: Components enforce a logical hierarchy for better SEO and A11y.
- **Customizable**: Every component supports the `.style()` modifier for atomic or shared styling.
