# Engine: Typography & Shaping Engine 🔤

The Typography Engine handles the most readable part of the UI. It transforms raw UTF-8 strings into a sequence of precisely positioned glyphs, accounting for complex language rules and high-performance hardware acceleration.

---

## 🏗️ Core Responsibilities

1.  **Text Shaping:** Uses **HarfBuzz** (via `cosmic-text`) to handle kerning, ligatures, and bidirectional text (LTR/RTL).
2.  **Line Breaking:** Implements modern algorithms for word-wrapping and multi-line paragraph layout.
3.  **Glyph Caching:** Manages a GPU **Texture Atlas** for glyphs. Instead of re-rendering a letter, it retrieves the pre-rasterized version from VRAM.
4.  **Rasterization:** High-precision anti-aliased font rendering via **Glyphon**.

---

## 🎨 Typographic Standards

*   **Sub-pixel Positioning:** Ensures text remains sharp even when moving or scaled by non-integer factors.
*   **Font Management:** Supports system fonts and custom `.ttf`/`.otf` assets.
*   **Agnosticism:** While GUI uses Glyphon, the TUI backend uses this engine to calculate line wraps before outputting raw ANSI characters.

---

## 🛠️ Internal Module Reference
- `crates/rupa-engine/src/renderer/gui/text_renderer.rs`: The WGPU-based text painter.
- `crates/rupa-vnode/src/style/typography.rs`: Data types for fonts, sizes, and weights.
