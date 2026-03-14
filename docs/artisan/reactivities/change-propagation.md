# Reactivity: Change Propagation 🔄

Rupa Framework follows a **Reactive Pipeline** that ensures data changes flow smoothly from the logic layer down to the hardware. This document tracks the journey of a single state change.

---

## 🔁 The Lifecycle of a Change

### 1. Signal Mutation (`rupa-signals`)
A component's reactive state is updated (e.g., `counter.set(v + 1)`). This mutation triggers the framework's reactive engine.

### 2. VNode Re-render (Build Phase)
The `rupa-signals` engine identifies the affected component and triggers its `render()` method. This produces a new **VNode** sub-tree representing the updated UI intent.

### 3. VNode Diffing (Diff Phase)
The `rupa-core` reconciliation engine compares the new VNode sub-tree with the previous snapshot. It identifies minimal structural and stylistic changes (e.g., text content change, style attribute update).

### 4. Patch Generation (Patch Phase)
The identified differences are converted into a set of **Patches** (Create, Update, Delete) and sent to the active renderer.

### 5. Hardware Execution (Render Phase)
The **Engine** (WGPU/TUI) or **Client** (DOM) consumes the patches:
- **Engine (Native):** Updates Taffy layout nodes and buffers new vertex data for the GPU.
- **Client (Web):** Performs surgical DOM manipulations via `web-sys`.
- **Final Output:** The hardware presents the updated frame.

---

## 🛡️ Reactive Integrity

- **Predictability:** Changes always flow in one direction (Signal -> Redraw -> Layout -> Paint).
- **Efficiency:** The framework does not "poll" for changes. If no signal changes, the GPU stays idle (0% CPU usage).
- **Atomicity:** Multiple signal changes within the same event handler will only trigger a single redraw request, preventing "frame stutter."
