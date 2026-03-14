# Components: Layout & Scaffolding 🏗️

Layout components are the scaffolding of your application. They provide the spatial structure needed to organize complex interfaces, from high-level regions to directional stacks.

---

## 🧠 Internal Anatomy

### 1. Structural Logic
These components are primarily containers for `Children`. They manage the **Spatial Scope** and the flow of their descendants.

### 2. The View Implementations
- **Section/Container:** Focus on structural boundaries and hardware clipping.
- **Stacks (V/H):** Focus on directional flow and item distribution via the Flex primitive.

---

## 🗝️ Standard Components

### `struct Div`
The most basic, "bare" container. It has no opinionated styles and renders as a standard `div` tag.

### `struct Section`
Used for major UI regions (e.g., "Sidebar", "Main").

### `struct Container`
A flexible box with scrolling support.

### `struct VStack` & `struct HStack`
Directional wrappers built on Flexbox. They simplify vertical and horizontal alignment by providing semantic defaults. 
- `VStack`: Defaults to `FlexDirection::Col` and `AlignItems::Stretch`.
- `HStack`: Defaults to `FlexDirection::Row` and `AlignItems::Center`.

### `struct Row` & `struct Col`
Lightweight, non-opinionated flex containers for horizontal and vertical flow respectively.

### `struct Fieldset`
A specialized container that renders a border with a text label at the top. Essential for creating structured TUI panels and form groups.

---

## 🎨 Common Layout Utilities
These components are the primary targets for:
- `.style(flex())`
- `.style(gap(f32))`
- `.style(items_center())`
