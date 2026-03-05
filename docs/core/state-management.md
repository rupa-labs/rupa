# State Management with Signals & Memos

Rupaui utilizes a **Signal-based Reactivity** system. Unlike traditional Virtual DOM frameworks, Signals provide fine-grained updates, while **Memos** ensure that derived data is efficiently cached.

## 📐 Why Signals & Memos?
- **High Performance**: Only components that consume the signal are recalculated.
- **Memoization (Caching)**: Derived states (Memos) only recalculate when their source signals change. This prevents memory pressure and expensive redundant computations.
- **WebAssembly Optimization**: Minimizes the overhead of syncing between Rust and the browser's DOM.

---

## 🚀 Basic Signal Usage

A `Signal` is a reactive container for any data type.

```rust
use rupaui::utils::Signal;

let count = Signal::new(0);
count.set(10); // Updates the value and its internal version
```

---

## 🧠 Derived State with Memos

A `Memo` is used for values that depend on other signals. It is similar to `useMemo` in React or `computed` in Vue/Solid.

```rust
use rupaui::utils::{Signal, Memo};

let count = Signal::new(5);

// Create a derived state: "double_count"
let double_count = Memo::new(count.clone(), |v| v * 2);

println!("{}", double_count.get()); // 10

count.set(10);
println!("{}", double_count.get()); // 20 (Recalculated because count changed)

// If we call .get() again without changing 'count', 
// it returns the cached 20 instantly!
println!("{}", double_count.get()); // 20 (Cached)
```

---

## 🏗 Best Practices
- Use **Signal** for state that you need to modify.
- Use **Memo** for any data that is calculated from a Signal (e.g., filtered lists, formatted strings, complex math).
- Memos help keep your `render()` functions fast and lightweight.
