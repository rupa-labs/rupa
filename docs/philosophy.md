# Rupa Framework Philosophy 🎨

Rupa Framework is a **modular meta-framework, cross-platform and multi-purpose**, engineered for artisans who demand the perfect balance between **Reactive Performance**, **Semantic Clarity**, and **Hardware-Accelerated Visuals**. Our philosophy is built on five core pillars.

---

## 1. Modular Meta-Framework (The Blueprint)
Rupa is designed as a **meta-framework**. Instead of a monolithic library, it provides an ecosystem of independent, low-level **Atoms** and functional **Composites**. This allows it to serve multiple purposes—from UI components to low-level reactivity engines—across any platform.

## 2. Utility-First, Semantic-Support
We believe functional utilities are unmatched for development speed, while semantic structures are essential for long-term scalability.
- **Utility:** Atomic styling through a functional API (e.g., `p(16.0)`, `bg(Color)`).
- **Semantic:** Components convey meaning and enforce hierarchy (e.g., `Navbar`, `Modal`, `Section`).

## 3. VNode Rendering Architecture (Agnostic Separation)
To ensure testability and multi-platform support, Rupa Framework enforces a strict architectural boundary via the **VNode (Virtual Node)** system:
- **Component (Logic):** Manages reactive state and event handling. It describes its UI intent by producing a VNode tree.
- **Renderer (Platform):** Consumes the VNode tree to perform platform-specific tasks (WGPU draw calls, ANSI character output, or DOM manipulation).
This separation allows components to remain 100% agnostic of the hardware they run on.

## 4. Signal-Based Fine-Grained Reactivity
Rupa Framework utilizes a **Signal** and **Memo** system. Instead of re-rendering the entire tree, only the specific components whose Signals have changed are triggered to re-run their `render()` method, producing a new VNode sub-tree for surgical patching.

## 5. Multi-Platform Hardware Acceleration
Rupa Framework is built for performance across every target:
- **Native GPU:** High-performance visuals via **WGPU** and custom shaders for SDF-based effects.
- **Terminal (TUI):** Optimized character-grid rendering with double-buffering and ANSI diffing.
- **Web (WASM):** Efficient DOM patching and WebGPU/WebGL hardware acceleration.
This ensures a premium, high-frame-rate experience whether you are building a 4K desktop app or an SSH-based CLI tool.

## 6. Visual DNA & Unified Scale
Aesthetic consistency is enforced through a unified 10-step scaling system (`Xs` to `Xl6`) applied globally across:
- **Spacing:** Margins and paddings.
- **Sizing:** Component dimensions and typography.
- **Layout:** Grid gaps and breakpoints.
This "DNA Visual" ensures that every element feels like part of a coherent, professional system by default.
