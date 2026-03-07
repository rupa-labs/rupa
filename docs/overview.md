# Project Overview 🎨

**Rupa Framework** is a modern, high-performance cross-platform UI framework for Rust. It is engineered for artisans who demand the perfect balance between **Semantic Structure** and **Utility-First Flexibility**.

Inspired by the ergonomics of TailwindCSS and the reliability of Bootstrap, Rupa Framework provides a type-safe, reactive engine built directly on top of hardware-accelerated primitives.

---

## 🌟 Key Features

- **Utility-First, Semantic-Support**: Compose complex visual identities using a functional API while maintaining a clean, meaningful component hierarchy.
- **Logic & View Pattern**: A strictly enforced separation between UI state (Logic) and rendering infrastructure (View), ensuring testability and modularity.
- **Signal-Based Reactivity**: Fine-grained state management using `Signal` and `Memo` for zero-overhead UI updates, automatically triggering hardware-accelerated redraws.
- **Hardware-Accelerated Rendering**: Built on **WGPU** with a specialized **2D Batch Renderer** for high-performance primitives (rects, shapes).
- **Industrial Layout Engine**: Powered by **Taffy**, providing full support for Flexbox and CSS Grid layouts.
- **Interactive Event System**: Full support for **Hit-Testing** and event dispatching (Click, Hover, Drag, Scroll) linked directly to the layout engine.
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
E --> F[Create Window winit]

F --> G[Initialize GPU Renderer]
G --> H[Initialize wgpu Device]
H --> I[Create Render Pipeline]

I --> J[Enter Event Loop]

J --> K{Event Type}

K -->|User Input| L[Platform Event Layer]
K -->|System Event| M[Window Resize / Close]
K -->|Redraw Request| N[Frame Pipeline]

L --> O[Event Dispatch System]
O --> P[Element Tree Event Propagation]
P --> Q[Component Event Handlers]

Q --> R[State Mutation]
R --> S[Mark Tree Dirty]
S --> T[Request Redraw]

T --> J

M --> U[Resize Renderer]
U --> J

N --> V[Clear Previous Layout]

V --> W[Build Element Tree]
W --> X[Create Element Nodes]

X --> Y[Layout Phase]

Y --> Z[Traverse Element Tree]
Z --> AA[Generate Taffy Nodes]
AA --> AB[Compute Layout Taffy]

AB --> AC[Render Phase]

AC --> AD[Traverse Element Tree]
AD --> AE[Generate Draw Commands]

AE --> AF[Batch Renderer Commands]

AF --> AG[Begin GPU Render Pass]

AG --> AH[Execute Batches]
AH --> AI[Draw Geometry]
AI --> AJ[Render Text]
AJ --> AK[Render SVG]

AK --> AL[Submit Command Buffer]

AL --> AM[Present Frame]

AM --> AN[Request Next Frame]

AN --> J
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
