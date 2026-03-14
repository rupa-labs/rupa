# Signals 📶

**Signals** are the primary source of truth in the Rupa Framework. They are reactive containers that hold a value and automatically notify anyone "listening" when that value changes.

---

## 🏗️ Creation
A signal can hold any type that implements `Clone`, `Send`, and `Sync`.

```rust
let count = Signal::new(0);
let name = Signal::new("Artisan".to_string());
```

---

## 🗝️ API Methods

| Method | Description |
| :--- | :--- |
| **`.get()`** | Returns a clone of the current value and tracks the dependency. |
| **`.set(val)`** | Overwrites the value and notifies all subscribers. |
| **`.update(f)`** | Mutates the value in-place using a closure. Efficient for collections. |
| **`.with(f)`** | Accesses the value by reference for reading without cloning. |

---

## 🧬 Dependency Tracking
The magic of signals lies in their ability to know **who** is using them. When you call `.get()` inside a `Memo`, `Effect`, or a Component's `render()` method, the signal automatically registers that caller as a subscriber.

```rust
let count = Signal::new(0);

// This Effect now 'belongs' to the count signal
Effect::new(move || {
    println!("The current count is: {}", count.get());
});

count.set(1); // Prints: "The current count is: 1"
```

---

## 📐 Thread Safety
Rupa Signals are built using atomic primitives and synchronized containers, making them safe to use in multi-threaded environments (like a GPU-accelerated desktop app).
