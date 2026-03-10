# Module: Reactive Network System (`crates/rupa-net`) 🌐

The `rupa-net` crate handles asynchronous Network I/O operations, ensuring they integrate seamlessly with Rupa's reactive update cycle.

---

## 1. Core Philosophy: Network as State
Network requests are treated as reactive state machines (Pending, Success, Error). When a request completes, it updates a `Signal`, automatically triggering a VNode reconciliation without manual callbacks.

## 2. Platform Implementations
- **Native (Desktop/Mobile)**: Built on top of `reqreq` or `hyper`.
- **Web (WASM)**: Bridges to the browser's native `fetch` API.

## 3. Key Exports (1:1 Mapping)
- `Resource<T>`: A reactive container for async data fetching.
- `Client`: The HTTP/WebSocket client configuration.
- `fetch()`: Utility to initiate network calls mapped to the `Resource`.
