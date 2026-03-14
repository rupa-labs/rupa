# Context Orchestrator Architecture 💉

The **Context Orchestrator** provides the Dependency Injection (DI) and Scoped Registry system for the Rupa Framework.

---

## 1. Core Pillars

- **Registry**: A thread-safe container for shared resources and services.
- **Scoping**: Allows resources to be provided to specific branches of the component tree.
- **Hooks**: Standardized `use_context` and `provide_context` APIs.

---

## 2. Technical Context

- **Decoupled Sharing**: Enables sharing data across distant components without "prop drilling."
- **Artisan Experience**: Designed to be lightweight and transparent.
