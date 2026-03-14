# Path: Zero Bloat (Pure Logic) 🥚

The **Zero Bloat** template is the most minimal starting point in the Rupa ecosystem. It is designed for Artisans who want absolute sovereignty over their architectural assembly.

---

## 🏗️ What's Inside?

- **Tier 1 Atoms Only**: Includes `rupa-base` and `rupa-signals`.
- **Zero Adapters**: No rendering engine or windowing system is pre-configured.
- **Pure Logic**: A clean `main.rs` ready for CLI logic or custom orchestrations.

---

## 🎯 Best For:

1.  **Back-end Services**: Building reactive systems that don't require a UI.
2.  **Specialized Tooling**: Custom data processors or automation scripts.
3.  **Experimental Assembly**: When you want to manually mix-and-match Rupa crates.

---

## 🚀 Quick Start

Initialize via CLI:
```bash
rupa create --template pure
```

### Manual Assembly (Cargo.toml)
```toml
[dependencies]
rupa-base = { git = "https://github.com/rupa-labs/rupa" }
rupa-signals = { git = "https://github.com/rupa-labs/rupa" }
```
