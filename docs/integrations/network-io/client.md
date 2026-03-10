# Module: Network Client (`crates/rupa-net/src/client.rs`) 🌐

This module defines the primary entry point for network configurations. It manages global headers, authentication tokens, and request timeouts.

---

## 🏗️ `struct Client`
The orchestrator for all outbound network requests.

- **Platform Adaptive**: Automatically selects `reqwest` for native or `web-sys::fetch` for WASM.
- **Middleware Support**: Allows injecting logic before requests (auth) and after responses (logging/telemetry).
