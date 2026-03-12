# `rupa-signals` ⚡

**The Reactive DNA.** This crate provides the foundational primitives for fine-grained reactivity in the Rupa Framework.

## 🛠️ Key Features

- **`Signal<T>`**: A thread-safe, reactive state container.
- **`Memo<T>`**: Derived state that only recomputes when dependencies change.
- **`Effect`**: Automatic side-effect execution triggered by signal changes.
- **`Runtime`**: A thread-local orchestration engine for tracking dependencies.

## 🚀 Usage

```rust
use rupa_signals::{Signal, Memo, Effect};

// 1. Create a Signal
let count = Signal::new(0);

// 2. Create a Memo (Derived State)
let double = Memo::new(move || count.get() * 2);

// 3. Create an Effect (Side Effect)
Effect::new(move || {
    println!("Count is: {}, Double is: {}", count.get(), double.get());
});

// 4. Update the state
count.set(5); // Automatically triggers the effect
```
