# Table 📊

A structural view for displaying grid-based, structured data sets.

---

## 🏗️ Purpose
`Table` organizes large amounts of information into readable rows and columns. It supports semantic headers and reactive row updates.

---

## 🚀 Example
```rust
Table::new()
    .headers(vec!["ID", "Name", "Status"])
    .rows(my_data_signal)
```
