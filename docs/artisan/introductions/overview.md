# Rupa Framework Overview 🎨

**Rupa Framework** is a modular meta-framework, cross-platform and multi-purpose, engineered for **Rust Artisans**. It represents a paradigm shift in how we build high-performance applications, moving away from monolithic "all-in-one" solutions toward a granular, **Atoms and Composites** model.

At its core, Rupa is built on the philosophy that a framework should be a collection of high-quality materials and tools that you can pick and choose from, rather than a rigid structure that dictates your entire workflow.

---

## 🏛️ The Vision: Why Rupa?

In the current Rust ecosystem, developers often face a binary choice: use a high-level, opinionated framework (like Leptos, Dioxus, or Slint) or build everything from scratch using low-level libraries (like WGPU, Winit, or Crossterm).

**Rupa fills the gap between these two worlds.** It provides the high-level ergonomics of a modern UI framework while maintaining the low-level transparency and modularity of a specialized toolkit. 

### The "Artisan" Mindset
We call our users **Artisans** because they care about the *craft*. They care about binary size, memory footprint, and architectural purity. Rupa is designed to honor that care by providing:
1.  **Zero-Cost Abstractions**: You only pay for what you use.
2.  **Agnostic Core**: Logic is separated from infrastructure, making your code future-proof.
3.  **Semantic Clarity**: Every API is designed to be expressive and intentional.

---

## 🧱 The Pillars of Rupa

Rupa is built upon three technological pillars that define its unique identity:

### 1. Fine-Grained Reactivity (The Pulse 🧬)
Unlike frameworks that rely on virtual DOM diffing for the entire application state, Rupa uses a **Signal-based reactivity engine**.
-   **Signals**: The source of truth. When a Signal changes, only the specific parts of the UI that depend on it are updated.
-   **Memos**: Derived state that caches results and only re-calculates when dependencies change.
-   **Effects**: Automatic side-effects that bridge reactive state to non-reactive worlds (like logging or file I/O).
-   **Result**: Surgical updates that bypass unnecessary reconciliation, leading to extreme performance even in complex UIs.

### 2. Universal UI Contract (The DNA 🌳)
Rupa defines UI through **VNodes (Virtual Nodes)**. A VNode is a platform-agnostic representation of an element, text, or component.
-   **Serializable**: VNodes can be serialized to JSON, enabling remote rendering and SSR.
-   **Agnostic**: A `VNode::element("button")` doesn't know if it's a WGPU mesh, a HTML `<button>`, or a Unicode box-drawing character.
-   **The Reconciler**: Rupa's core kernel takes these VNodes, compares them, and issues minimal **Patches** to the platform-specific renderer.

### 3. Hexagonal 3-Tier Architecture (The Workshop 🛠️)
Rupa follows a strict **Atoms and Composites** model to ensure zero-coupling between logic and hardware:
-   **Tier 1: Atoms (Materials)**: Pure invariants (Signals, VNode types, Base types). No platform dependencies.
-   **Tier 2: Composites (The Master's Craft)**: The Kernel. Orchestrates Atoms into systems (Reconciler, Action Bus, Component Lifecycle).
-   **Tier 3: Showrooms (The Finished Showroom)**: Hardware adapters (Desktop/WGPU, Web/DOM, Terminal/Crossterm).

---

## 🏪 The Multi-Platform Showrooms

Rupa is truly multi-purpose. By swapping the **Showroom** (Tier 3), you can target entirely different environments with the same **Composite** logic.

### 🖥️ Rupa Desktop
Powered by **WGPU**, Rupa Desktop provides hardware-accelerated 2D graphics. It includes a custom batch renderer that can handle thousands of primitives per frame with sub-millisecond latency.
-   **Layout**: Full Flexbox and CSS Grid support via **Taffy**.
-   **Text**: High-quality text shaping and rendering via **Glyphon**.

### 🌐 Rupa Web
Targeting **WebAssembly (WASM)**, Rupa Web offers two modes:
-   **Native DOM**: Maps VNodes directly to HTML elements for best accessibility and SEO.
-   **WGPU/Canvas**: Renders the desktop-quality graphics engine inside a browser tab.

### 📟 Rupa Terminal
For CLI/TUI lovers, Rupa Terminal uses **Crossterm** to render UIs in the terminal using Unicode characters and ANSI colors. It supports reactive layouts and interactive components inside a terminal buffer.

### 📱 Rupa Mobile
Native integration for **iOS (Swift/Metal)** and **Android (Kotlin/Vulkan)**, allowing you to share 90% of your UI logic between desktop and mobile.

---

## 🎨 Aesthetic & Design System

Rupa isn't just about code; it's about **Aesthetics**.
-   **OKLCH Color Space**: Native support for perceptually uniform colors, ensuring that your themes look consistent across different brightness levels.
-   **Motion & Physics**: Built-in spring physics and physics-based transitions (`rupa-motion`) that make UIs feel "alive" and responsive.
-   **Semantic API**: Functional styling that reads like English: `.with_style(Style::new().p(16).bg(Color::Blue))`.

---

## 🚀 The Execution Pipeline

How Rupa transforms your code into a living application:

1.  **Build Phase**: Your components call `render()`, producing a tree of **VNodes**.
2.  **Diff Phase**: The **Reconciler** compares the new tree with the previous one.
3.  **Patch Phase**: A list of **Patches** (e.g., `UpdateText`, `MoveNode`, `SetStyle`) is generated.
4.  **Layout Phase**: **Taffy** calculates the physical dimensions and positions of every node.
5.  **Render Phase**: The **Showroom** (GPU/DOM/Terminal) executes the patches and draws the final frame.

---

## 🛠️ The Artisan's Toolkit

Rupa comes bundled with independent sub-systems that you can pull into any project:
-   **`rupa-store`**: Reactive persistent state.
-   **`rupa-net`**: Agnostic async networking.
-   **`rupa-i18n`**: Fine-grained internationalization.
-   **`rupa-auth`**: Pluggable authentication logic.
-   **`rupa-telemetry`**: Built-in tracing and performance monitoring.

---

## ⚖️ Comparison: Rupa vs. Others

| Feature | Rupa | Leptos / Dioxus | Slint |
| :--- | :--- | :--- | :--- |
| **Architecture** | Hexagonal (Atoms/Composites) | Web-centric / Component-centric | DSL-centric |
| **Reactivity** | Fine-grained Signals | Signals / Virtual DOM | Property Binding |
| **Platform** | Native GPU, Web, TUI, Mobile | Web, Desktop (WebView) | Native UI Kits, GPU |
| **Granularity** | Extremely High (Meta-framework) | High (Framework) | High (Framework) |
| **Philosophy** | Artisan / Modular | Declarative / Web-like | Performance / Tooling |

---

*Rupa is not just a framework; it is a commitment to the craft of software engineering. Welcome to the workshop.*
