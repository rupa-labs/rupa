# Module: GUI Text Renderer (`gui/text_renderer.rs`) 🔤

The Text Renderer is the typography engine of Rupa Framework. It bridges the gap between UTF-8 strings and hardware-accelerated glyph rendering.

---

## 🧠 Internal Anatomy

### 1. Shaping & Atlas Management
- **Cosmic-Text Integration:** Uses `FontSystem` and `SwashCache` to handle font discovery and glyph shaping.
- **Glyphon Bridge:** Orchestrates the `TextAtlas` which stores pre-rasterized glyphs in GPU textures.

### 2. Viewport Projection
Automatically calculates the correct projection matrix so that text scales and moves perfectly with the rest of the UI geometry.

---

## 🗝️ API Anatomy

- `prepare()`: Pre-computes glyph positions and uploads new characters to the atlas.
- `render()`: Executes the WGPU render pass for all shaped text areas.

---

## 🛡️ Correctness
By using HarfBuzz (via cosmic-text), Rupa Framework correctly supports ligatures, kerning, and complex scripts (Arabic, Devanagari, etc.), which are often broken in simpler UI frameworks.
