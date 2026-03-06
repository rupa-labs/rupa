# Rupaui Documentation Index

Welcome to the **Rupaui** documentation. Rupaui is a modern, cross-platform UI framework built with Rust, designed for artisans who value semantic structure and utility-first flexibility.

## 🚀 Architecture Pipeline

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

## 🏗 Core Framework
- [Philosophy](./core/philosophy.md)
- [State Management](./core/state-management.md)
- [Plugins](./core/plugins.md)
- [Extending Rupaui](./core/extending.md)
- [Platforms](./core/platforms.md)
- [Control Flow](./core/control-flow.md)
- [Attributes](./core/attributes.md)
- [Vector Math](./core/vector-math.md)

## 🎨 DNA Visual (Styling)
- [Styling Overview](./styling/styling.md)
- [Theme Engine](./styling/theme.md)
- [Modular Styling](./styling/modular-styling.md)
- [Spacing & Sizing](./styling/spacing-sizing.md)
- [Background & Border](./styling/background-border.md)
- [Typography](./styling/typography.md)
- [Effects & Shadows](./styling/effects.md)
- [Filters](./styling/filters.md)
- [Motion & Transform](./styling/motion-transform.md)
- [Interactivity & SVG](./styling/interactivity-svg.md)
- [Layout System](./styling/layout.md)
- [Utility Helpers](./styling/helpers.md)
- [Variants](./styling/variants.md)
- [Tables](./styling/tables.md)

## 🧩 Components
- [Brand](./components/brand.md)
- [Div](./components/div.md)
- [Elements](./components/elements.md)
- [Forms](./components/forms.md)
- [Layout](./components/layout.md)
- [Section](./components/section.md)
- [SVG Drawing](./components/svg-drawing.md)
- [Text](./components/text.md)
- [Theme Switcher](./components/theme-switcher.md)
- [Viewport](./components/viewport.md)
- [Window](./components/window.md)
