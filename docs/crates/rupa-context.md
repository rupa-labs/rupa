# `rupa-context` 💉

**The Dependency Injection Port.** A scoped, reactive registry for providing data to the VNode tree.

## 🛠️ Key Features

- **`provide_context<T>`**: Injects a value into the current scope.
- **`use_context<T>`**: Retrieves a value from the current or parent scope.
- **`Registry`**: A scoped container supporting hierarchical lookups.
- **`thread_local` Isolation**: Safe and transparent context management during rendering.

## 🚀 Usage

```rust
use rupa_context::{provide_context, use_context};

// 1. Provide data at a high level
provide_context(Theme::dark());

// 2. Retrieve data in a child component
if let Some(theme) = use_context::<Theme>() {
    println!("Current theme: {:?}", theme);
}
```
