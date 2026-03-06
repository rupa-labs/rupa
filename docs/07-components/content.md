# API Reference: Content & Disclosure 📦

Containers used for structuring data and managing the visual density of the interface.

---

## 🏗️ Card
### `Card::new() -> Self`
A surfaced grouping element.

| Method | Argument | Description |
| :--- | :--- | :--- |
| `.header(Box<dyn Component>)` | Component | Attaches a top section. |
| `.child(Box<dyn Component>)`  | Component | Appends a child to the card body. |
| `.footer(Box<dyn Component>)` | Component | Attaches a bottom section. |

---

## 🏗️ Accordion
### `Accordion::new(title: impl Into<String>, expanded: Signal<bool>) -> Self`
A collapsible content box.

| Method | Argument | Description |
| :--- | :--- | :--- |
| `.child(Box<dyn Component>)` | Component | Appends a child to the collapsible area. |

---

## 🏗️ Table
### `Table::new(headers: Vec<String>, rows: Vec<TableRow>) -> Self`
A grid component for structured data.

| Method | Argument | Description |
| :--- | :--- | :--- |
| `TableRow { cells }` | `Vec<String>` | Standard struct for row data. |

---

## 🎨 Common Layout (Stylable)
- `.style(p(f32))` used to set inner gutters.
- `.style(bg(Color))` to change surface background.
