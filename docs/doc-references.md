# Rupa Framework Documentation Index 📚

Welcome to the **Rupa Framework Open Blueprint**. This document serves as the central directory for all technical documentation, organized by system modules.

---

## 🏁 Getting Started & Vision
Foundational documents for understanding Rupa Framework's purpose and standards.

| Document | Description |
| :--- | :--- |
| **[Project Overview](./overview.md)** | High-level vision, key features, and core tech stack. |
| **[Philosophy](./philosophy.md)** | The "Artisan Pillars" and the reasoning behind Rupa Framework's design. |
| **[Engineering Standards](./engineering-standards.md)** | Mandatory coding conventions, naming rules, and SOC mandates. |
| **[Architecture Overview](./architecture.md)** | A conceptual map of the framework infrastructure. |
| **[Platform References](./platform-references.md)** | Standardized entry-points for Desktop, Web, Terminal, and Mobile. |

---

## 🏗️ Technical References
Comprehensive indices for components, utilities, and infrastructure.

| Document | Description |
| :--- | :--- |
| **[Component Reference](./component-references.md)** | Index of all UI elements from Layout to Overlays. |
| **[Support Reference](./support-references.md)** | Index of Math, Reactivity, A11y, and Error handling utilities. |

---

## 🔬 System Blueprints
Detailed technical specifications for every internal framework module.

### Platform Integrations
| Module | Description |
| :--- | :--- |
| **[Platform Orchestrator](./integrations/platform-orchestrator.md)** | App lifecycle management and the Agnostik Bridge. |
| **[Input Events](./integrations/input-events.md)** | Universal event schema across all platforms. |
| **[Input Dispatcher](./integrations/input-dispatcher.md)** | Event normalization and hit-testing logic. |
| **[Desktop Runner](./integrations/desktop-runner.md)** | Winit & WGPU shell for Desktop (macOS, Win, Linux). |
| **[Terminal Runner](./integrations/terminal-runner.md)** | Crossterm shell for CLI/TUI applications. |
| **[Web Runner](./integrations/web-runner.md)** | WASM & Canvas shell for browser environments. |
| **[Mobile Runner](./integrations/mobile-runner.md)** | Android & iOS shell with lifecycle management. |

### Rendering Systems
| Module | Description |
| :--- | :--- |
| **[Renderer Interface](./renderings/renderer-interface.md)** | The universal contract for all visual backends. |
| **[GUI Backend](./renderings/gui-backend.md)** | Aggregation of WGPU-specific rendering logic. |
| **[TUI Renderer](./renderings/tui-renderer.md)** | ANSI/Character-grid based terminal painter. |

### Geometric Layouts
| Module | Description |
| :--- | :--- |
| **[Scene Core](./layouts/scene-core.md)** | The spatial Single Source of Truth (SSOT). |
| **[Layout Engine](./layouts/layout-engine.md)** | Integration with Taffy for Flexbox and Grid. |
| **[Scene Node](./layouts/scene-node.md)** | Platform-agnostic handle for geometric objects. |

### Reactivity Engine
| Module | Description |
| :--- | :--- |
| **[Signals & Memos](./reactivity/signals.md)** | The reactive nucleus of the framework. |
| **[Fine-Grained Updates](./reactivity/fine-grained-updates.md)** | Performance strategy for targeted UI refreshes. |

### Component Architectures
| Module | Description |
| :--- | :--- |
| **[Component Trait](./architectures/component-trait.md)** | The core contract for all UI elements. |
| **[VNode Architecture](./architectures/vnode.md)** | The universal rendering bridge between logic and pixels. |
| **[View Core](./architectures/view-core.md)** | Anatomical standard for component infrastructure. |
| **[Logic & View Pattern](./architectures/logic-and-view.md)** | Strict Separation of Concerns (SOC) standard. |
| **[Module Standard](./architectures/module-standard.md)** | Directory and naming conventions for elements. |

### UI Primitives (Atomic)
| Module | Description |
| :--- | :--- |
| **[Primitive Design](./primitives/primitive-design.md)** | The design standard for Layer 6 atomic blocks. |
| **[Overlay](./primitives/overlay.md)** | Absolute positioning and z-index management. |

### Semantic Components (Artisan)
| Module | Description |
| :--- | :--- |
| **[Component Design](./components/component-design.md)** | The design standard for Layer 7 semantic elements. |
| **[Theme Control](./components/theme-switcher.md)** | Standardized Light/Dark mode switching logic. |

### App Compositions
| Module | Description |
| :--- | :--- |
| **[App Bootstrap](./compositions/app-bootstrap.md)** | Orchestration logic from App::new() to Runner::run(). |
| **[Control Flow](./compositions/control-flow.md)** | Logic components for conditional and list rendering. |
| **[Routing](./compositions/routing.md)** | Universal navigation and URL synchronization system. |

### Ecosystems & Visual DNA
| Module | Description |
| :--- | :--- |
| **[Styling API](./ecosystems/styling-api.md)** | Functional "Utility-First" API reference. |
| **[Theme Engine](./ecosystems/theme-engine.md)** | DNA Visual standard for consistent aesthetics. |
| **[Color Math](./ecosystems/color-math.md)** | OKLCH standards for perceptual uniformity. |
