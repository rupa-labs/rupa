# Style Utility Reference 🎨

This document provides a comprehensive list of all style utilities and modifiers available in Rupa Framework. These utilities allow you to customize the appearance and layout of your components using a fluent, chainable API.

---

## 📏 Spacing Utilities
Control the inner and outer space of your components.

| Utility | Description | Example |
| :--- | :--- | :--- |
| **p(val)** | Sets padding on all four sides. | `.p(16.0)` |
| **px(val)** | Sets horizontal padding (left and right). | `.px(24.0)` |
| **py(val)** | Sets vertical padding (top and bottom). | `.py(8.0)` |
| **m(val)** | Sets margin on all four sides. | `.m(12.0)` |
| **mx(val)** | Sets horizontal margin (left and right). | `.mx(auto)` |
| **my(val)** | Sets vertical margin (top and bottom). | `.my(10.0)` |

---

## ✨ Visual Utilities
Enhance the look and feel with colors and shapes.

| Utility | Description | Example |
| :--- | :--- | :--- |
| **bg(color)** | Sets the background color. | `.bg(Color::Semantic("surface".into(), None))` |
| **text_color(color)** | Sets the text color. | `.text_color(Color::Rgba(1.0, 1.0, 1.0, 1.0))` |
| **rounded(val)** | Sets the border radius. | `.rounded(8.0)` |
| **rounded_full()** | Makes the corners fully rounded (pill/circle). | `.rounded_full()` |

---

## 🏗️ Layout Utilities
Define how components are positioned and displayed.

| Utility | Description | Example |
| :--- | :--- | :--- |
| **flex()** | Sets the display mode to Flexbox. | `.flex()` |
| **col()** | Sets the flex direction to Column. | `.col()` |
| **row()** | Sets the flex direction to Row. | `.row()` |
| **gap(val)** | Sets the space between children in a flex or grid. | `.gap(12.0)` |
| **items_center()** | Aligns items along the cross axis to the center. | `.items_center()` |
| **justify_center()** | Aligns content along the main axis to the center. | `.justify_center()` |

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

## 🖱️ Interactive States (Pseudo-classes)
Conditional modifiers that apply styles only when a specific interaction occurs.

| Utility | Description | Example |
| :--- | :--- | :--- |
| **hover(mod)** | Applies styles when the cursor is over the element. | `.style(hover(bg(Color::Blue)))` |
| **active(mod)** | Applies styles while the element is being pressed. | `.style(active(scale(0.95)))` |
| **focus(mod)** | Applies styles when the element has keyboard focus. | `.style(focus(outline(2.0)))` |
| **group_hover(mod)** | Applies styles when any element in the parent group is hovered. | `.style(group_hover(text_color(Color::White)))` |

---

## 📱 Responsive Design (10-Step Breakpoints)
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
    .col() // Default (xs)
    .md(row()) // Switch to horizontal on tablets
    .xl3(gap(100.0)) // Wider gap on Full HD and above
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
