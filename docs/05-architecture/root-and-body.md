# Infrastructure: Root & Body 🏛️

The Root and Body elements are the silent foundation of every Rupaui application. They are created automatically when you instantiate an `App` and call `.run()`.

---

## 🏗️ The Root (Metadata Container)

The **Root** is not a visual element, but an architectural container managed by the `App` struct. It holds the high-level configuration of your application.

### Key Responsibilities
- **Metadata Management**: Stores `AppMetadata` (Title, Version, Author, etc.).
- **Platform Sync**: Synchronizes the metadata with the Operating System (e.g., setting the window title).
- **Orchestration**: Manages the transition from bootstrap to the event loop.

### Public API (via `App`)
```rust
App::new("My App")
    .title("High Fidelity UI")
    .version("1.0.0")
    .author("Reasnov")
```

---

## 🕺 The Body (Implicit Container)

The **Body** is the internal root-level visual component. It is the equivalent of the `<body>` tag in HTML.

### Key Characteristics
- **Always Present**: You don't need to declare it; Rupaui wraps your `.root()` component inside a `Body` automatically.
- **Fixed Dimensions**: It always fills 100% of the window width and height (`w_full`, `h_full`).
- **Surface Layer**: It is the primary layer for global background colors and main layout alignment.

### Styling the Body
You can style the Body element directly via the `App` object to set a global background or padding:

```rust
App::new("Demo")
    .style(bg(Color::Semantic("background".into(), None)))
    .root( ... )
```

---

## 🔄 Interaction Flow

1. **OS Window**: Created by the HAL (Layer 1).
2. **Body**: Attached to the Window by the `PlatformCore`. It takes the physical dimensions of the window.
3. **User Root**: Injected as the single child of the `Body`.

This separation ensures that your application always has a stable, full-window anchor, even if your specific root component is small or unstyled.
