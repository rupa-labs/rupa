# `rupa-net` 🌐

**The Network Port.** This crate provides **Atoms** for reactive, asynchronous networking and resource management. It bridges the gap between the I/O world and the reactive UI.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Typed responses and strict error boundaries prevent malformed data from reaching the UI.
    - **Sustain (S2)**: `Resource<T>` state machine simplifies async handling (Loading/Ready/Error).
    - **Scalable (S3)**: Decoupled `Client` port prevents framework lock-in to specific HTTP libraries.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Resource<T>`** | Reactive async state. | State machine: `Uninitialized`, `Loading`, `Ready`, `Error`. |
| **`Client`** | The Network Port. | Trait defining GET/POST contracts. |
| **`MockClient`** | Testing Implementation. | Returns pre-configured responses for headless tests. |
| **`use_asset`** | Asset hook. | Reactive helper for loading bytes from paths. |

## 🚀 Usage

```rust
use rupa_net::{Resource, MockClient, Client};

// 1. Define a reactive resource
let weather = Resource::<WeatherData>::new_signal();

// 2. Perform async fetch (Implemented by Showroom)
let client: Arc<dyn Client> = Arc::new(MockClient { response: vec![] });
tokio::spawn(async move {
    let result = client.get("https://api.rupa.rs/weather").await;
    weather.set_result(result);
});

// 3. React to state machine in UI
Effect::new({
    let weather = weather.clone();
    move || match weather.get() {
        Resource::Loading => println!("Fetching..."),
        Resource::Ready(data) => println!("Temp: {}", data.temp),
        Resource::Error(e) => println!("Fail: {}", e),
        _ => {}
    }
});
```

## 🧪 Testing & Reliability
- **Mock Client**: Built-in `MockClient` enables deterministic testing of network-dependent components without real internet access.
- **TDD Support**: Async state transitions are verified to be thread-safe and reactively consistent.
- **Zero-Dependency**: No dependency on `reqwest` or `curl`; the Atom only defines the *Port* contract.
