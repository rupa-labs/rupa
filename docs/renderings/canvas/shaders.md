# Module: Shader Management (`crates/rupa-canvas/src/shaders.rs`) 🌌

This module handles the loading and compilation of WGSL shader modules.

---

## 🏗️ Logic
- **SDF Shaders**: Built-in support for Signed Distance Field rendering (perfect for smooth UI shapes).
- **Custom Shaders**: Allows developers to inject their own `.wgsl` files into the pipeline.
- **Uniforms**: Provides a reactive bridge to update shader uniforms via Signals.
