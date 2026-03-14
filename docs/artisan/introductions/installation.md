# Installation & Environment Setup 🛠️

Welcome to the Rupa Framework. Before you can begin crafting multi-platform masterpieces, you must prepare your workshop with the necessary tools and system dependencies.

---

## 🧪 Global Prerequisites

Regardless of your target platform, every Rupa Artisan needs the following:

1.  **Rust Toolchain**: Rupa is built with modern Rust. Ensure you have the latest stable version installed via [rustup.rs](https://rustup.rs/).
    ```bash
    rustup update stable
    ```
2.  **Cargo**: The Rust package manager (included with the toolchain).

---

## 🎨 The Artisan CLI (The Workshop Key)

The `rupa` command-line tool is your gateway to scaffolding, building, and managing projects.

### 1. Install from Source (Current Method)
While we prepare for our official crates.io release, you can install the CLI directly from our repository:
```bash
cargo install --git https://github.com/rupa-labs/rupa rupa-cli
```

### 2. Verify the Installation
Check if the tool is ready for use:
```bash
rupa --version
```

---

## 🖥️ Platform-Specific Requirements

Rupa bridges the gap between different hardware environments. Each environment requires specific system-level materials.

### 1. Desktop Adapter (High-Performance GPU)
Targeting Linux, Windows, or macOS requires libraries for windowing and hardware-accelerated graphics (WGPU).

#### **Linux (Ubuntu/Debian)**
```bash
sudo apt update
sudo apt install libwayland-dev libx11-dev pkg-config libfontconfig1-dev libasound2-dev
```

#### **Linux (Fedora)**
```bash
sudo dnf install wayland-devel libX11-devel fontconfig-devel alsa-lib-devel
```

#### **macOS**
Ensure you have the **Xcode Command Line Tools** installed:
```bash
xcode-select --install
```

#### **Windows**
Ensure you have the **Visual Studio Build Tools** installed with the "Desktop development with C++" workload selected.

---

### 2. Terminal Adapter (Aesthetic TUI)
The Terminal Adapter is the most lightweight target and requires **zero** additional system dependencies. It works on any modern terminal emulator that supports **TrueColor (24-bit ANSI)**.

- **Recommended Terminals**: Alacritty, Kitty, WezTerm, iTerm2, or Windows Terminal.

---

### 3. Web Adapter (WASM Excellence)
To target the browser, you must prepare your system for WebAssembly (WASM) compilation.

#### **Install the WASM Target**
```bash
rustup target add wasm32-unknown-unknown
```

#### **Install WASM-Pack (Optional but Recommended)**
We recommend using `wasm-pack` for advanced builds, though the Rupa CLI can handle basic web scaffolding automatically.
```bash
cargo install wasm-pack
```

---

## 📱 Mobile Adapter (Experimental)
Mobile support is currently in the experimental "Artisan Preview" phase.

- **Android**: Requires the **Android NDK** and `cargo-apk`.
- **iOS**: Requires **Xcode** and the `aarch64-apple-ios` target.

---

## 🚀 Creating Your First Project

Once your environment is ready, you can use the interactive wizard to craft your first application:

```bash
rupa create
```

Choose **"Showroom (Zero Bloat)"** for a minimal starting point, or **"Native Power"** for a full-featured desktop template.

---

## 🔄 Keeping Tools Refined

The workshop is constantly evolving. To update your CLI to the latest version:

```bash
rupa update --canary
```

*Your environment is now primed. It is time to start crafting.*
