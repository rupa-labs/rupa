# Module: Project Templates (`crates/rupa-cli/src/templates.rs`) 📂

This module manages the boilerplate code and project structures used during scaffolding.

---

## 🏗️ Logic
- **Remote Fetching**: Ability to pull templates from Git repositories.
- **Variable Injection**: Replaces placeholders (e.g., `{{app_name}}`) in source files using `AppMetadata`.
- **Pre-sets**: Default configurations for "Desktop-Only," "SSR-Web," and "Mobile-First" layouts.
