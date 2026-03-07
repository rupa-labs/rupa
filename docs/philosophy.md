# Rupa Framework Philosophy 🎨

Rupa Framework is engineered for artisans who demand the perfect balance between **Reactive Performance**, **Semantic Clarity**, and **Hardware-Accelerated Visuals**. Our philosophy is built on five core pillars.

---

## 1. Utility-First, Semantic-Support
We believe functional utilities are unmatched for development speed, while semantic structures are essential for long-term scalability.
- **Utility:** Atomic styling through a functional API (e.g., `p(16.0)`, `bg(Color)`).
- **Semantic:** Components convey meaning and enforce hierarchy (e.g., `Navbar`, `Modal`, `Section`).

## 2. Logic & View Separation (SOC)
To ensure testability and modularity, Rupa Framework enforces a strict architectural boundary within every component:
- **Logic (The Brain):** Pure state management, reactive signals, and event handling. It is decoupled from rendering infrastructure.
- **View (The Body):** Visual metadata, layout nodes, and GPU paint instructions.
This separation allows for easier unit testing of logic and swapping visual implementations without breaking the "brain" of the component.

## 3. Signal-Based Fine-Grained Reactivity
Rupa Framework utilizes a **Signal** and **Memo** system. Instead of re-rendering the entire tree, only the specific components tied to a changed Signal are marked as `dirty`. This ensures zero-overhead updates and maintains a consistent 60+ FPS even in complex UIs.

## 4. Hardware-Accelerated Rendering (WGPU)
Rupa Framework is built directly on **WGPU**, bypassing traditional CPU-bound rendering. Every primitive—rectangles, text, and vector paths—is processed via custom shaders on the GPU. This enables high-performance visual effects like real-time SDF shadows, rounded corners, and perceptual color blending without taxing the main thread.

## 5. Visual DNA & Unified Scale
Aesthetic consistency is enforced through a unified 10-step scaling system (`Xs` to `Xl6`) applied globally across:
- **Spacing:** Margins and paddings.
- **Sizing:** Component dimensions and typography.
- **Layout:** Grid gaps and breakpoints.
This "DNA Visual" ensures that every element feels like part of a coherent, professional system by default.
