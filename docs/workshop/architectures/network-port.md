# Network Port Architecture 🌐

The **Network Port** provides the agnostic interface for asynchronous network communication.

---

## 1. Core Duties

- **Resource Fetching**: Standardized async requests (HTTP, WebSocket).
- **Agnostic Client**: An interface that can be implemented by platform-specific fetchers (e.g., reqwest on Desktop, web-sys on Web).
- **State Integration**: Seamlessly maps network responses into reactive Signals.
