# Module: Platform Bundler (`crates/rupa-cli/src/bundler.rs`) 📦

This module handles the heavy lifting of converting a Rust project into a ready-to-install application.

---

## 🏗️ Targets
- **Android**: Generates `.apk` / `.aab` using the NDK.
- **Web**: Packages WASM and JS glue code into a deployable bundle.
- **Desktop**: Packages binaries into AppImage, MSI, or DMG formats.
