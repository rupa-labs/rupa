# Module: GUI Text Renderer (`crates/rupa-engine/src/renderer/gui/text_renderer.rs`) 🔤

The Text Renderer is the hardware-accelerated typography engine for the Rupa Framework. It bridges the gap between agnostic `VNode::Text` nodes and the high-performance Glyphon rendering pipeline.

---

## 🧠 Internal Anatomy

### 1. Shaping & Atlas Management
- **Cosmic-Text Integration:** Uses `FontSystem` and `SwashCache` to handle font discovery and glyph shaping.
- **Glyphon Bridge:** Orchestrates the `TextAtlas` which stores pre-rasterized glyphs in GPU textures.

### 2. Viewport Projection
Automatically calculates the correct projection matrix so that text scales and moves perfectly with the rest of the VNode tree geometry.

---

## 🗝️ API Anatomy

- **`prepare()`**: Pre-computes glyph positions for the current VNode tree and uploads new characters to the GPU atlas.
- **`render()`**: Executes the WGPU render pass for all shaped text areas during the frame's presentation.

---

## 🛡️ Correctness
By using HarfBuzz (via cosmic-text), the Rupa Framework correctly supports ligatures, kerning, and complex scripts (Arabic, Devanagari, etc.), ensuring high-quality typography on all targets.
