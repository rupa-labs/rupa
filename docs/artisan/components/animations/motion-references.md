# Motion & Animation API Reference ✨

This document is the formal index of all animation and transition units provided by the Rupa Framework.

---

## 🏗️ Core Transitions
| Method | Argument | Description |
| :--- | :--- | :--- |
| **`.transition(props)`** | `&str` | Enables animation for specific properties. |
| **`.transition_duration(ms)`**| `u32` | Sets animation length in milliseconds. |
| **`.transition_delay(ms)`** | `u32` | Sets start delay in milliseconds. |
| **`.transition_repeat(n)`** | `u32` | Number of loops (0 = infinite). |
| **`.ease_in()`** | - | Accelerating timing curve. |
| **`.ease_out()`** | - | Decelerating timing curve. |
| **`.ease_in_out()`** | - | Symmetric timing curve. |
| **`.linear()`** | - | Constant speed timing curve. |

---

## 🚪 Lifecycle Animations (Entry/Exit)
| Method | Argument | Execution Point |
| :--- | :--- | :--- |
| **`.transition_enter_start(m)`**| `StyleModifier`| Style before mounting. |
| **`.transition_enter_end(m)`** | `StyleModifier`| Style after mounting completes. |
| **`.transition_leave_start(m)`**| `StyleModifier`| Style when unmounting triggers. |
| **`.transition_leave_end(m)`** | `StyleModifier`| Style before physical removal. |

---

## 🎨 Artisan Presets
| Method | Argument | Effect Description |
| :--- | :--- | :--- |
| **`.pulse()`** | - | Infinite subtle scale breathing. |
| **`.pulse_opacity()`** | - | Infinite subtle opacity breathing. |
| **`.spin()`** | - | Infinite 360° rotation. |
| **`.spin_reverse()`** | - | Infinite CCW 360° rotation. |
| **`.fade_in()`** | - | Opacity 0.0 -> 1.0 transition. |
| **`.fade_out()`** | - | Opacity 1.0 -> 0.0 transition. |

---

## 💫 Advanced Motion (Physics)
| Method | Argument | Description |
| :--- | :--- | :--- |
| **`.spring()`** | - | Applies default elastic physics. |
| **`.spring_custom(m, s, d)`** | `f32, f32, f32` | Custom mass, stiffness, and damping. |
