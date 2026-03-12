# `rupa-net` 🌐

**The Network Port.** Reactive primitives for asynchronous I/O and data fetching.

## 🛠️ Key Features

- **`Resource<T>`**: A state machine for async data (`Loading`, `Ready`, `Error`).
- **`Fetch`**: Background task for updating resources.
- **`fetch_queued`**: Offloads network requests to a central task queue.

## 🚀 Usage

```rust
use rupa_net::Resource;

// 1. Create a reactive resource
let user_data = Resource::<User>::new_signal();

// 2. Fetch data (spawns background task)
Resource::fetch(user_data.clone(), async {
    let api_result = my_api_call().await;
    api_result
});

// 3. React to state in UI
Effect::new(move || {
    match user_data.get() {
        Resource::Loading => println!("Loading..."),
        Resource::Ready(u) => println!("Hello {}", u.name),
        Resource::Error(e) => println!("Error: {}", e),
        _ => {}
    }
});
```
