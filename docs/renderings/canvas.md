# Module: Canvas & Custom Graphics (`crates/rupa-canvas`) 🎨

The `rupa-canvas` crate provides low-level access to the GPU for drawing custom shapes, animations, and high-performance visualizations.

---

## 1. Core Philosophy: Native Primitive Access
When standard UI elements (Div/Flex) are not enough, Canvas provides a blank slate with direct hardware acceleration via WGPU.

## 2. Module Structure (1:1 Mapping)
- `api.rs`: The fluent 2D/3D drawing API (lines, curves, meshes).
- `shaders.rs`: Management of custom WGSL shader modules.
- `pipeline.rs`: Integration logic for inserting custom draw calls into the `rupa-engine` loop.
