# Installation Guide 🛠️

This guide covers the two primary ways to install the Rupa Framework CLI and integrate its materials into your projects.

---

## 🎨 Option 1: Via Crates.io (Recommended)

The easiest way to get the Rupa CLI is through the official Rust package registry. This provides you with the `rupa` command globally.

### 1. Install the CLI
```bash
cargo install rupa-cli
```

### 2. Verify Installation
```bash
rupa --version
```

### 3. Usage in Projects
Add Rupa to your `Cargo.toml` dependencies:
```toml
[dependencies]
rupa = "0.1"
```

---

## 🛠️ Option 2: Via Git Repository (Artisan/Bleeding Edge)

If you want the latest unreleased features without manually cloning the repository, you can install the CLI directly from GitHub:

```bash
cargo install --git https://github.com/rupa-labs/rupa rupa-cli
```

### Option 3: Via Manual Git Clone (Contributor Mode)
If you intend to contribute to the framework or modify its materials:
...
### 3. Local Development Link
If you are building an app while modifying the framework itself, link to your local clone in your app's `Cargo.toml`:
```toml
[dependencies]
rupa = { path = "../path/to/rupa/crates/rupa" }
```

---

## 🧪 System Prerequisites

Depending on your target platform, you may need additional system-level materials:

### Desktop (WGPU / Winit)
- **Linux**: Requires Wayland/X11 development headers.
  - `sudo apt install libwayland-dev libx11-dev pkg-config` (Ubuntu/Debian)
  - `sudo dnf install wayland-devel libX11-devel` (Fedora)
- **Windows/macOS**: Standard build tools (Visual Studio or Xcode).

### Terminal (TUI)
- No additional dependencies required beyond a modern terminal emulator supporting ANSI colors.

---

## 🚀 Next Step
Once installed, launch the [**Installation Wizard**](./tooling/create-rupa-app.md) to craft your first project:
```bash
rupa create
```

---

## 🔄 Updating the CLI

To ensure you have the latest artisan tools, you should occasionally update the `rupa` command.

### The Artisan Way (Recommended)
Simply run the update command from within the CLI itself:
```bash
rupa update           # Update to the latest stable version
rupa update --canary  # Get the absolute latest (bleeding edge) from Git
rupa update --to 0.2.0 # Switch to a specific version
```

### Manual Methods
If the internal update is not available, you can use these alternatives:
...
