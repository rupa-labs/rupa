# Rupaui Documentation Index 🎨

Welcome to the **Rupaui Open Blueprint**. This documentation is structured to mirror the 9-layer architecture of the framework, providing total transparency from hardware abstraction to design system composition.

---

## 🏁 Getting Started
- [Project Overview](./overview.md) - Vision, key features, and tech stack.
- [Philosophy](./philosophy.md) - Design principles and the 5 Artisan Pillars.
- [Engineering Standards](./engineering-standards.md) - Coding conventions and mandates.
- [Architecture Overview](./architecture.md) - Conceptual map of the 9-layer stack.

---

## 🏗️ The 9-Layer Blueprint

### [01] Hardware Abstraction (HAL)
- [Platform Orchestrator](./01-hal/platform-orchestrator.md) - `mod.rs` (App lifecycle)
- [Input Events](./01-hal/input-events.md) - `events.rs` (Universal schema)
- [Input Dispatcher](./01-hal/input-dispatcher.md) - `dispatcher.rs` (Normalization)
- [GUI Runner](./01-hal/gui-runner.md) - `gui/mod.rs` (Winit shell)
- [GUI Window](./01-hal/gui-window.md) - `gui/window.rs` (Window wrapper)
- [TUI Runner](./01-hal/tui-runner.md) - `tui/mod.rs` (Terminal shell)
- [TUI Terminal](./01-hal/tui-terminal.md) - `tui/terminal.rs` (Crossterm wrapper)

### [02] Rendering Engine
- [Renderer Interface](./02-rendering/renderer-interface.md) - `mod.rs` (Universal contract)
- [GUI Backend](./02-rendering/gui-backend.md) - `gui/mod.rs` (Aggregation)
- [GUI Renderer](./02-rendering/gui-renderer.md) - `gui/renderer.rs` (WGPU orchestration)
- [GUI Batcher](./02-rendering/gui-batcher.md) - `gui/batcher.rs` (Buffer optimization)
- [GUI Pipeline](./02-rendering/gui-pipeline.md) - `gui/pipeline.rs` (WGPU state)
- [GUI Text Renderer](./02-rendering/gui-text-renderer.md) - `gui/text_renderer.rs` (Typography)
- [GUI Texture](./02-rendering/gui-texture.md) - `gui/texture.rs` (VRAM assets)
- [TUI Renderer](./02-rendering/tui-renderer.md) - `tui/mod.rs` (ANSI painter)

### [03] Geometric Scene Layer
- [Scene Core](./03-layout/scene-core.md) - `mod.rs` (Spacial SSOT)
- [Layout Engine](./03-layout/layout-engine.md) - `layout.rs` (Taffy brain)
- [Scene Node](./03-layout/scene-node.md) - `node.rs` (Agnostic handle)
- [Spatial Awareness](./03-layout/spatial-awareness.md) - Hit-testing logic

### [04] Reactivity Layer
- [Signals & Memos](./04-reactivity/signals.md) - `state.rs` (Reactive nucleus)
- [Change Propagation](./04-reactivity/change-propagation.md) - Execution pipeline
- [Fine-Grained Updates](./04-reactivity/fine-grained-updates.md) - Optimization strategy

### [05] Component Architecture
- [Component Trait](./05-architecture/component-trait.md) - `component.rs` (The Contract)
- [View Core](./05-architecture/view-core.md) - `view.rs` (Anatomical standard)
- [Element Tree](./05-architecture/element-tree.md) - `element_tree.rs` (Traversal)
- [Plugin System](./05-architecture/plugin-system.md) - `plugin.rs` (Extensions)
- [Logic & View Pattern](./05-architecture/logic-and-view.md) - Conceptual SOC standard
- [Module Standard](./05-architecture/module-standard.md) - Directory conventions

### [06] UI Primitives
- [Primitive Design](./06-primitives/primitive-design.md) - Standard for L6
- [Div](./06-primitives/div.md) - Basic block primitive
- [Flex](./06-primitives/flex.md) - 1D layout primitive
- [Grid](./06-primitives/grid.md) - 2D layout primitive
- [Overlay](./06-primitives/overlay.md) - Absolute positioning

### [07] Semantic Components
- [Artisan Design](./07-components/component-design.md) - Standard for L7
- [Button](./07-components/button.md) - Triggers
- [Form Controls](./07-components/forms.md) - Input
- [Feedback](./07-components/feedback.md) - Status
- [Navigation](./07-components/navigation.md) - Landmarks
- [Content & Disclosure](./07-components/content.md) - Data grouping
- [Overlays](./07-components/overlay.md) - Modals
- [Layout Scaffolding](./07-components/layout.md) - Structural containers
- [VStack](./07-components/vstack.md) - Vertical Stacks
- [HStack](./07-components/hstack.md) - Horizontal Stacks
- [Graphics](./07-components/svg.md) - Vector & Icons
- [Identity](./07-components/brand.md) - Brand
- [Theme Control](./07-components/theme-switcher.md) - Modes

### [08] Composition Layer
- [App Bootstrap](./08-composition/app-bootstrap.md) - Orchestration logic
- [Control Flow](./08-composition/control-flow.md) - `control_flow.rs` (Logic components)
- [Viewports](./08-composition/viewports.md) - `viewport.rs` (Cameras)

### [09] Ecosystem & Design System
- [Styling API](./09-ecosystem/styling-api.md) - Functional utilities
- [Theme Engine](./09-ecosystem/theme-engine.md) - DNA Visual
- [Color Math](./09-ecosystem/color-math.md) - OKLCH Standards
- [Scale System](./09-ecosystem/scale-system.md) - 10-step scaling
- [Plugin System](./09-ecosystem/plugins.md) - `plugin.rs` (Extensibility)
