# Asset Port Architecture 📦

The **Asset Port** manages the loading, caching, and distribution of application resources (Images, Fonts, Data).

---

## 1. Core Systems

- **Asset Manager**: Central hub for requesting resources.
- **Loaders**: Pluggable adapters for different resource types or protocols.
- **Reactive Assets**: Returns a Signal that resolves once the asset is loaded.

---

## 2. Technical Context

- **Async Orchestration**: Assets are loaded in the background to ensure UI fluidity.
- **Cross-Platform**: Handles assets from local files, remote URLs, or embedded binary data.
