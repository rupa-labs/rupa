# Engine: Path & Vector Engine (SVG) 📐

The Vector Engine ensures that logos, icons, and custom drawings remain infinitely sharp at any zoom level. It focuses on rendering mathematical paths (Bezier curves) directly on the GPU.

---

## 🏗️ Core Responsibilities

1.  **SDF Path Rendering:** Renders complex vector paths using **Signed Distance Fields**. This allows for resolution-independent sharpness and efficient zooming.
2.  **Bezier Resolution:** Decomposes quadratic and cubic Bezier curves into GPU-friendly data structures.
3.  **SVG Support:** Parses a subset of SVG commands (MoveTo, LineTo, CurveTo) into Rupa Framework's internal path format.

---

## 📐 Mathematical Standards

*   **Non-Raster Zooming:** When using the `Viewport` zoom, the Vector Engine does not scale pixels; it recalculates the distance field, preserving edge crispness.
*   **Path Composition:** Supports filling and stroking (outlining) paths with varying thickness and perceptual OKLCH colors.

---

## 🛠️ Internal Module Reference
- `crates/rupa-engine/src/renderer/vector.rs`: (Planned) The Bezier-to-SDF compiler.
- `crates/rupa-core/src/svg.rs`: Standard path and point data structures.
