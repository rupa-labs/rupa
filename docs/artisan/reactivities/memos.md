# Memos 🧠

**Memos** are derived reactive values. They compute a result based on other signals or memos and cache that result until one of their dependencies changes.

---

## 🏗️ Creation
A memo is created by providing a closure that performs a calculation.

```rust
let count = Signal::new(10);

// This memo re-calculates only when 'count' changes
let double_count = Memo::new(move || count.get() * 2);
```

---

## 🗝️ Why use Memos?

1.  **Caching (Memoization)**: If you read a memo 100 times, the calculation only runs **once**. Subsequent reads return the cached value.
2.  **Surgical Updates**: If a memo's calculation results in the same value as before (e.g., a boolean check), its own subscribers will **not** be notified, preventing unnecessary UI updates.
3.  **Clean Logic**: Memos allow you to keep your component's `render()` method clean by moving complex data transformations into declarative derivations.

---

## 🚀 Example: Filtered List
```rust
let search_query = Signal::new("".to_string());
let all_items = Signal::new(vec!["Apple", "Banana", "Cherry"]);

let filtered_items = Memo::new(move || {
    let query = search_query.get().to_lowercase();
    all_items.get().into_iter()
        .filter(|i| i.to_lowercase().contains(&query))
        .collect::<Vec<_>>()
});
```

*Memos are the "intelligent" parts of your application's brain. Use them to ensure your logic is both fast and elegant.*
