# Style Utility Reference 🎨

This document provides a comprehensive list of all style utilities and modifiers available in Rupa Framework. These utilities allow you to customize the appearance and layout of your components using a fluent, chainable API.

---

## 📏 Spacing Utilities
Control the inner and outer space of your components. These utilities support both raw `f32` values and the Rupa Framework `Scale` system.

| Utility | Description | Example |
| :--- | :--- | :--- |
| **p(val)** | Sets padding on all four sides. | `.p(16.0)` or `.p(Scale::Md)` |
| **px(val)** | Sets horizontal padding (left and right). | `.px(24.0)` |
| **py(val)** | Sets vertical padding (top and bottom). | `.py(8.0)` |
| **pt(val)** | Sets padding top. | `.pt(4.0)` |
| **pr(val)** | Sets padding right. | `.pr(4.0)` |
| **pb(val)** | Sets padding bottom. | `.pb(4.0)` |
| **pl(val)** | Sets padding left. | `.pl(4.0)` |
| **gap(val)** | Sets the space between children in a flex or grid. | `.gap(12.0)` |

---

## ✨ Visual Utilities
Enhance the look and feel with colors and shapes.

| Utility | Description | Example |
| :--- | :--- | :--- |
| **bg(color)** | Sets an explicit background color. | `.bg(Color::Rgba(1.0, 0.0, 0.0, 1.0))` |
| **bg_primary()** | Sets the brand primary background. | `.bg_primary()` |
| **bg_surface()** | Sets the default surface background. | `.bg_surface()` |
| **text_color(color)** | Sets the text color. | `.text_color(Color::White)` |
| **font_size(val)** | Sets the font size in pixels. | `.font_size(16.0)` |
| **font_bold()** | Sets font weight to bold. | `.font_bold()` |
| **rounded(val)** | Sets the border radius. | `.rounded(8.0)` |
| **rounded_full()** | Makes the corners fully rounded (pill/circle). | `.rounded_full()` |

---

## 🏗️ Layout Utilities
Define how components are positioned and displayed.

| Utility | Description | Example |
| :--- | :--- | :--- |
| **flex()** | Sets the display mode to Flexbox. | `.flex()` |
| **grid()** | Sets the display mode to CSS Grid. | `.grid()` |
| **flex_row()** | Sets flex direction to Row. | `.flex_row()` |
| **flex_col()** | Sets flex direction to Column. | `.flex_col()` |
| **items_center()** | Aligns items along the cross axis. | `.items_center()` |
| **justify_center()** | Aligns content along the main axis. | `.justify_center()` |
| **justify_between()**| Distributes items evenly (main axis). | `.justify_between()` |
| **absolute()** | Sets position to Absolute. | `.absolute()` |
| **relative()** | Sets position to Relative. | `.relative()` |
| **z(val)** | Sets the stacking order (Z-index). | `.z(10)` |

---

## 📐 Sizing Utilities
Set explicit or relative dimensions for your elements.

| Utility | Description | Example |
| :--- | :--- | :--- |
| **w(val)** | Sets an explicit width. | `.w(200.0)` |
| **h(val)** | Sets an explicit height. | `.h(100.0)` |
| **w_full()** | Makes the component fill its parent's width. | `.w_full()` |
| **h_full()** | Makes the component fill its parent's height. | `.h_full()` |

---

## 🔡 Typography Utilities
Refine the appearance of text elements. These can be used directly on the `Text` component or as part of a `Style` object.

| Utility | Description | Example |
| :--- | :--- | :--- |
| **size(val)** | Sets the font size in pixels. | `.size(24.0)` |
| **thin()** | Sets font weight to 100. | `.thin()` |
| **light()** | Sets font weight to 300. | `.light()` |
| **normal()** | Sets font weight to 400. | `.normal()` |
| **medium()** | Sets font weight to 500. | `.medium()` |
| **semibold()** | Sets font weight to 600. | `.semibold()` |
| **bold()** | Sets font weight to 700. | `.bold()` |
| **black()** | Sets font weight to 900. | `.black()` |
| **italic()** | Makes the text italic. | `.italic()` |
| **underline()** | Adds an underline decoration. | `.underline()` |
| **strikethrough()**| Adds a line-through decoration. | `.strikethrough()` |
| **uppercase()** | Converts text to UPPERCASE. | `.uppercase()` |
| **lowercase()** | Converts text to lowercase. | `.lowercase()` |
| **capitalize()** | Capitalizes the first letter. | `.capitalize()` |
| **left()** | Aligns text to the left. | `.left()` |
| **center()** | Aligns text to the center. | `.center()` |
| **right()** | Aligns text to the right. | `.right()` |
| **justify()** | Justifies the text. | `.justify()` |
| **mono()** | Uses a monospaced font family. | `.mono()` |

---

## 🎨 Artisan Presets (Quick Styles)
High-level presets for rapid development.

| Preset | Description |
| :--- | :--- |
| **h1() .. h6()** | Heading presets from 48px down to 16px. |
| **lead()** | Large, light text for introduction sections. |
| **small()** | Small, muted text for secondary info. |
| **primary()** | Sets text color to brand primary. |
| **success()** | Sets text color to success green. |
| **warning()** | Sets text color to warning amber. |
| **error()** | Sets text color to error red. |
| **muted()** | Sets text color to neutral gray. |
| **dim()** | Sets text color to gray with 60% opacity. |

---

## 🖱️ Interactive States (Pseudo-classes)
Conditional modifiers that apply styles only when a specific interaction occurs.

| Utility | Description | Example |
| :--- | :--- | :--- |
| **hover(mod)** | Applies styles when the cursor is over the element. | `.style(hover(bg(Color::Blue)))` |
| **active(mod)** | Applies styles while the element is being pressed. | `.style(active(scale(0.95)))` |
| **focus(mod)** | Applies styles when the element has keyboard focus. | `.style(focus(outline(2.0)))` |
| **group_hover(mod)** | Applies styles when any element in the parent group is hovered. | `.style(group_hover(text_color(Color::White)))` |

---

## 📱 Responsive Design (10-Scale Breakpoints)
Apply conditional styles based on the window width. Rupa Framework follows a mobile-first approach with 10 granular steps.

| Breakpoint | Min-Width | Description | Example |
| :--- | :--- | :--- | :--- |
| **xs(mod)** | 0px | Mobile (Extra Small) | `.xs(p(12.0))` |
| **sm(mod)** | 640px | Large Mobile (Small) | `.sm(p(24.0))` |
| **md(mod)** | 768px | Tablet (Medium) | `.md(row())` |
| **lg(mod)** | 1024px | Laptop (Large) | `.lg(gap(40.0))` |
| **xl(mod)** | 1280px | Desktop (Extra Large) | `.xl(w(800.0))` |
| **xl2(mod)** | 1536px | Large Desktop | `.xl2(w(1000.0))` |
| **xl3(mod)** | 1920px | Full HD Monitors | `.xl3(w(1400.0))` |
| **xl4(mod)** | 2560px | 2K / QHD Monitors | `.xl4(w(1800.0))` |
| **xl5(mod)** | 3840px | 4K / UHD Monitors | `.xl5(w(2400.0))` |
| **xl6(mod)** | 5120px | 5K / Ultra-wide | `.xl6(w(3200.0))` |

### Usage Example
```rust
VStack::new()
    .flex_col() // Default (xs)
    .md(flex_row()) // Switch to horizontal on tablets
    .xl3(gap(Scale::Xl)) // Wider gap on Full HD and above
```

---

## 🌗 Theme Utilities
Manage global visual states like color modes.

| Utility | Description | Example |
| :--- | :--- | :--- |
| **Theme::toggle_mode()** | Toggles between Light and Dark modes. | `Theme::toggle_mode();` |
| **Theme::set_mode(mode)** | Sets an explicit color mode (Light/Dark). | `Theme::set_mode(ColorMode::Light);` |

---

## 🛠️ Advanced Usage: Style Objects
For reusable design patterns, you can declare standalone `Style` objects.

```rust
let my_custom_style = Style::new()
    .bg(Color::Semantic("primary".into(), None))
    .p(20.0)
    .rounded(12.0);

// Apply to any component
VStack::new()
    .style(my_custom_style)
    .child(...)
```
