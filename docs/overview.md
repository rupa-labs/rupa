# Project Overview 🎨

**Rupa Framework** is a modern, high-performance cross-platform UI framework for Rust. It is engineered for artisans who demand the perfect balance between **Semantic Structure** and **Utility-First Flexibility**.

Inspired by the ergonomics of TailwindCSS and the reliability of Bootstrap, Rupa Framework provides a type-safe, reactive engine built directly on top of hardware-accelerated primitives.

---

## 🌟 Key Features

- **Utility-First, Semantic-Support**: Compose complex visual identities using a functional API while maintaining a clean, meaningful component hierarchy.
- **VNode Rendering Architecture**: A universal virtual tree structure that decouples components from the platform, enabling multi-target rendering (WGPU, TUI, WASM, SSR).
- **Atoms & Composites Workspace**: A highly modular codebase organized into specialized crates, ensuring zero-cost abstractions and independent scalability.
- **Signal-Based Reactivity**: Fine-grained state management using `Signal` and `Memo` for zero-overhead UI updates, automatically triggering hardware-accelerated redraws.
- **Hardware-Accelerated Rendering**: Built on **WGPU** with a specialized **2D Batch Renderer** for high-performance primitives (rects, shapes).
- **Industrial Layout Engine**: Powered by **Taffy**, providing full support for Flexbox and CSS Grid layouts.
- **Interactive Event System**: Full support for **Hit-Testing** and event dispatching (Click, Hover, Drag, Scroll, Touch) linked directly to the layout engine.
- **Artisan Color System**: Native support for **OKLCH** color space for perceptually uniform aesthetics and precise theme control.

---

## 🚀 Execution Pipeline

The following diagram illustrates the lifecycle of a Rupa Framework application, from initialization to the reactive render loop.

```mermaid
flowchart TD

A[Application Start] --> B[Create App Instance]

B --> C[Register Plugins]
C --> D[Build Root Component]

D --> E[Initialize Event Loop]
E --> F[Platform Shell (Winit/Mobile/WASM)]

F --> G[Initialize Graphics Backend (WGPU/TUI)]
G --> H[Initialize Hardware Device]
H --> I[Create Pipeline State]

I --> J[Enter Event Loop]

J --> K{Event Type}

K -->|User Input| L[Input Dispatcher]
K -->|System Event| M[Resize / Suspend / Resume]
K -->|Redraw Request| N[Reactive Render Loop]

L --> O[Hit-Test Scene]
O --> P[Capture & Bubble Events]
P --> Q[Component Event Handlers]

Q --> R[Signal State Mutation]
R --> S[Mark Component Dirty]
S --> T[Request Redraw]

T --> J

M --> U[Platform Orchestrator Actions]
U --> J

N --> V[Build Phase]

V --> W[Component render() calls]
W --> X[Generate New VNode Tree]

X --> Y[Diff Phase]

Y --> Z[Compare Old vs New VNodes]
Z --> AA[Identify Minimal Patches]

AA --> AB[Patch Phase]

AB --> AC[Layout Update (Taffy)]
AC --> AD[Geometry & Text Batching]

AD --> AE[GPU Submission]

AE --> AF[Begin Graphics Pass]

AF --> AG[Execute Command Buffers]
AG --> AH[Draw Geometry]
AH --> AI[Render Text]
AI --> AJ[Render SVG]

AJ --> AK[Submit & Present]

AK --> AL[Request Next Frame]

AL --> J
```

---

## 🛠 Tech Stack

Rupa Framework stands on the shoulders of the Rust ecosystem's most powerful crates:

| Layer | Technology |
| :--- | :--- |
| **Graphics API** | [WGPU](https://wgpu.rs/) |
| **Layout Algorithm** | [Taffy](https://github.com/DioxusLabs/taffy) |
| **Windowing & Events** | [Winit](https://github.com/rust-windowing/winit) |
| **Text Rendering** | [Glyphon](https://github.com/grovesNL/glyphon) |
| **Reactivity** | Custom Signal/Memo Engine |
| **Color Math** | OKLCH (Custom Implementation) |
