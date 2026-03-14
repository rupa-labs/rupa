# Style Utility API Reference 🎨

This document is the exhaustive formal index of all fluent styling units provided by the Rupa Framework.

---

## 📏 Spacing Utilities
| Method | Argument | Target Property |
| :--- | :--- | :--- |
| **`.p(val)`** | `impl Into<Unit>` | All Padding |
| **`.px(val)`** | `impl Into<Unit>` | Left & Right Padding |
| **`.py(val)`** | `impl Into<Unit>` | Top & Bottom Padding |
| **`.pt(val)`** | `impl Into<Unit>` | Top Padding |
| **`.pr(val)`** | `impl Into<Unit>` | Right Padding |
| **`.pb(val)`** | `impl Into<Unit>` | Bottom Padding |
| **`.pl(val)`** | `impl Into<Unit>` | Left Padding |
| **`.m(val)`** | `impl Into<Unit>` | All Margin |
| **`.mx(val)`** | `impl Into<Unit>` | Left & Right Margin |
| **`.my(val)`** | `impl Into<Unit>` | Top & Bottom Margin |
| **`.gap(val)`** | `impl Into<Unit>` | Flex/Grid Gap |

---

## 📐 Sizing Utilities
| Method | Argument | Target Property |
| :--- | :--- | :--- |
| **`.w(val)`** | `impl Into<Unit>` | Width |
| **`.h(val)`** | `impl Into<Unit>` | Height |
| **`.w_full()`** | - | Width: 100% |
| **`.h_full()`** | - | Height: 100% |
| **`.size(w, h)`** | `Into<Unit>, Into<Unit>` | Set both W & H |
| **`.min_w(val)`** | `impl Into<Unit>` | Minimum Width |
| **`.max_w(val)`** | `impl Into<Unit>` | Maximum Width |

---

## 🏗️ Layout & Positioning
| Method | Argument | Target Property |
| :--- | :--- | :--- |
| **`.flex()`** | - | Display: Flex |
| **`.grid()`** | - | Display: Grid |
| **`.flex_row()`** | - | Direction: Horizontal |
| **`.flex_col()`** | - | Direction: Vertical |
| **`.cols(n)`** | `u16` | Grid Columns |
| **`.rows(n)`** | `u16` | Grid Rows |
| **`.items_start()`** | - | Cross-axis: Start |
| **`.items_center()`** | - | Cross-axis: Center |
| **`.items_end()`** | - | Cross-axis: End |
| **`.items_stretch()`**| - | Cross-axis: Stretch |
| **`.justify_start()`** | - | Main-axis: Start |
| **`.justify_center()`**| - | Main-axis: Center |
| **`.justify_end()`** | - | Main-axis: End |
| **`.justify_between()`**| - | Main-axis: Space Between |
| **`.justify_around()`** | - | Main-axis: Space Around |
| **`.absolute()`** | - | Position: Absolute |
| **`.relative()`** | - | Position: Relative |
| **`.z(val)`** | `i32` | Stacking Order |

---

## ✨ Visual Decorators
| Method | Argument | Target Property |
| :--- | :--- | :--- |
| **`.bg(color)`** | `Color` | Background Color |
| **`.bg_primary()`** | - | Theme Primary Background |
| **`.bg_surface()`** | - | Theme Surface Background |
| **`.rounded(val)`** | `impl Into<Unit>` | Border Radius |
| **`.rounded_full()`**| - | Fully Rounded (Pill) |
| **`.text_color(c)`** | `Color` | Typography Color |

---

## 🔡 Typography Decorators
| Method | Argument | Weight/Style |
| :--- | :--- | :--- |
| **`.thin()`** | - | Weight: 100 |
| **`.light()`** | - | Weight: 300 |
| **`.normal()`** | - | Weight: 400 |
| **`.medium()`** | - | Weight: 500 |
| **`.semibold()`** | - | Weight: 600 |
| **`.bold()`** | - | Weight: 700 |
| **`.black()`** | - | Weight: 900 |
| **`.italic()`** | - | Style: Italic |
| **`.underline()`** | - | Decoration: Underline |
| **`.strikethrough()`**| - | Decoration: Line-through |
| **`.uppercase()`** | - | Transform: UPPERCASE |
| **`.lowercase()`** | - | Transform: lowercase |
| **`.capitalize()`** | - | Transform: Capitalize |
| **`.left()`** | - | Align: Left |
| **`.center()`** | - | Align: Center |
| **`.right()`** | - | Align: Right |
| **`.justify()`** | - | Align: Justify |
| **`.mono()`** | - | Family: Monospace |

---

## 🎨 Artisan Quick-Presets
High-level semantic shortcuts for rapid styling.

| Preset | Applied Styles |
| :--- | :--- |
| **`.h1()` .. `.h6()`** | Heading sizes and weights (48px down to 16px). |
| **`.lead()`** | Large, light introduction text. |
| **`.small()`** | Small, muted secondary text. |
| **`.primary()`** | Text Color: Brand Primary. |
| **`.success()`** | Text Color: Semantic Success. |
| **`.warning()`** | Text Color: Semantic Warning. |
| **`.error()`** | Text Color: Semantic Danger. |
| **`.muted()`** | Text Color: Neutral Muted Gray. |
| **`.dim()`** | Text Color: Low Opacity Gray. |

---

## 🖱️ Interaction States
| Method | Argument | Context |
| :--- | :--- | :--- |
| **`.on_hover_style(m)`**| `StyleModifier` | Applied on Pointer Hover |
| **`.on_active_style(m)`**| `StyleModifier` | Applied while Pressed |
| **`.on_focus_style(m)`** | `StyleModifier` | Applied when Focused |

---

## 📱 Responsive Breakpoints
| Method | Min-Width | Target Device |
| :--- | :--- | :--- |
| **`.xs(mod)`** | 0px | Mobile (Extra Small) |
| **`.sm(mod)`** | 640px | Large Mobile (Small) |
| **`.md(mod)`** | 768px | Tablet (Medium) |
| **`.lg(mod)`** | 1024px | Laptop (Large) |
| **`.xl(mod)`** | 1280px | Desktop (Extra Large) |
| **`.xl2(mod)`** | 1536px | Large Desktop |
| **`.xl3(mod)`** | 1920px | Full HD Monitors |
| **`.xl4(mod)`** | 2560px | 2K / QHD Monitors |
| **`.xl5(mod)`** | 3840px | 4K / UHD Monitors |
| **`.xl6(mod)`** | 5120px | 5K / Ultra-wide |
