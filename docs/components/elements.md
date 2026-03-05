# UI Elements Reference

Rupaui features a vast library of semantic components, mapping modern design patterns to high-performance Rust logic.

## 🔘 Action & Input
- `Button`, `ButtonGroup`, `CloseButton`: Standard action triggers.
- `Input`, `Select`, `Check`, `Range`: Reactive form controls.

## 📢 Feedback & Status
- `Alert`, `Badge`: Contextual notifications and labels.
- `Progress`: Progress bar driven by a `Signal<f32>`.
- `Spinner`: Visual loading indicator.
- `Placeholder`: Skeleton screens for asynchronous loading.

---

## 🧭 Navigation
- `Navbar`: Top-level branding and navigation area.
- `Nav`: Lists of navigation links.
- `Pagination`: Interface for multi-page data.
- `Breadcrumb`: Hierarchical path tracking.

---

## 📦 Content & Disclosure
- `Card`: Grouped content container.
- `Accordion`: Collapsible content groups.
- `ListGroup`: Flexible list containers.
- `Collapse`: Reactive container for toggling visibility.
- `Dropdown`: Contextual disclosure menu.

---

## 🎭 Overlays (High Priority)
- `Modal`: Centered overlay blocking interactions.
- `Offcanvas`: Sliding sidebar overlay.
- `Toast`: Temporary floating notification.

```rust
let is_modal_open = Signal::new(false);

Modal::new(is_modal_open.clone())
    .child(Box::new(Text::new("Confirm Action")))
    .child(Box::new(Button::new("Cancel")));
```

## 🗝 Implementation Standards
- **Signal-First**: Visibility (`Modal`, `Collapse`) and values (`Progress`, `Input`) are bound to reactive Signals.
- **Semantic Structure**: Components enforce a logical hierarchy for better SEO and A11y.
- **Customizable**: Every component supports the `.style()` modifier for atomic or shared styling.
