# Module: Validation Schemas (`crates/rupa-forms/src/schema.rs`) 📜

This module provides a declarative way to define validation rules for complex objects.

---

## 🏗️ Logic
- **Structure Mapping**: Maps validation rules to specific fields in a Rust struct.
- **Cross-field Validation**: Handles dependencies between fields (e.g., "Password" and "Confirm Password" must match).
