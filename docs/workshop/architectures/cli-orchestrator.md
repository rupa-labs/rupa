# CLI Orchestrator Architecture 🛠️

The **CLI Orchestrator** (the `rupa-cli` crate) provides the developer experience tooling for the Rupa Framework.

---

## 1. Core Commands

- **`create`**: Scaffolds new Rupa projects with recommended workspace structures.
- **`build`**: Manages the compilation and optimization pipeline for different **Showrooms**.
- **`dev`**: Starts a local development server with Hot Module Replacement (HMR) capabilities.

---

## 2. Technical Context

Acts as the external **Artisan Tool** that automates the setup of the **Atoms and Composites** architecture.
