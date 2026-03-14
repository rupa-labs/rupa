# `rupa-cli` 🛠️

**The Artisan Tooling.** This crate provides the developer experience (DX) utilities to scaffold, build, and manage Rupa projects.

---

## 🏛️ Architectural Role
- **Category**: Developer Tooling
- **Identity**: The Artisan's Workbench
- **Responsibility**: Automates the setup of the **Atoms and Composites** architecture, manages build pipelines for multiple Showrooms, and provides aesthetic terminal feedback.

## 🛠️ Key Commands
- `rupa create <name>`: Scaffolds a new project using standard templates.
- `rupa build <showroom>`: Compiles the project for a specific target (desktop, web, terminal).
- `rupa dev`: Starts a local development server with hot-reloading (where supported).

## 🚀 Usage

```bash
# Install the CLI
cargo install rupa-cli

# Create a new Full-Stack application
rupa create my_app --template fullstack

# Run the terminal showroom
rupa run terminal
```
