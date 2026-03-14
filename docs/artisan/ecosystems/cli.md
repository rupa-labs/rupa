# Module: Rupa CLI (`crates/rupa-cli`) 🚀

The `rupa-cli` crate is the primary developer interface for the Rupa Framework. It automates project scaffolding, development workflows, and cross-platform builds.

---

## 1. Core Philosophy: Zero-Boilerplate DX
Developers should focus on artisan UI logic, not platform configuration. The CLI abstracts the complexities of Rust targets (WASM, Android, Desktop).

## 2. Module Structure (1:1 Mapping)
- `commands.rs`: Implementation of subcommands (`new`, `dev`, `build`).
- `templates.rs`: Management of project stencils and boilerplates.
- `bundler.rs`: Logic for packaging applications into platform-native formats.
- `config.rs`: Handles `rupa.toml` application manifest parsing.
