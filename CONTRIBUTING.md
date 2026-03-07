# Contributing to Rupa Framework 🎨

Thank you for your interest in contributing to **Rupa Framework**! We are building a modern, high-performance UI framework for Rust, and we welcome contributions from artisans of all skill levels.

---

## 🛠️ Engineering Standards

Before you start, please read our [Engineering Standards](./docs/engineering-standards.md). Adherence to these principles is mandatory for all contributions:

1.  **Separation of Concerns (SOC):** Always follow the **Logic & View** pattern for components.
2.  **Agnostic Bridge:** Ensure that core logic does not depend on platform-specific libraries (WGPU/Winit/Crossterm).
3.  **Documentation 1:1:** Every new module in `src/` must have a corresponding `.md` file in `docs/` explaining its anatomy and API.
4.  **Semantic Naming:** Use clear, descriptive names that focus on intent.

---

## 🚀 Development Workflow

1.  **Fork the Repository:** Create your own branch from `main` or the latest version branch (e.g., `v0`).
2.  **Set up Environment:** Ensure you have the latest Rust toolchain installed.
3.  **Implement & Test:** Write clean code and include automated tests.
4.  **Sync Docs:** Update the relevant documentation files in `docs/`.
5.  **Submit a PR:** Provide a clear description of your changes and why they are necessary.

---

## 🧩 Modifying Components

If you are adding or changing a component in `src/elements/`:
- Ensure it composes **`ViewCore`** for its visual state.
- Implement the **`Component`** and **`Stylable`** traits.
- Ensure the `paint` method is backend-agnostic.

---

## ⚖️ Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](./CODE_OF_CONDUCT.md).

We look forward to your contributions! Happy Coding!
