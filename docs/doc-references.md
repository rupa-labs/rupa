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

## 🏪 Artisan Showrooms (Tier 3)
Specialized entry points pre-assembled for specific business use cases.

| Document | Description |
| :--- | :--- |
| **[Showroom Overview](./facades/overview.md)** | The philosophy of specialized bundles. |
| **[Rupa Desktop](./facades/desktop.md)** | Pre-composed bundle for Native GUI apps. |
| **[Rupa Web](./facades/web.md)** | Pre-composed bundle for WASM/SPA frontend apps. |
| **[Rupa Server](./facades/server.md)** | Pre-composed bundle for SSR and Backend services. |
| **[Rupa TUI](./facades/tui.md)** | Pre-composed bundle for terminal tools. |
| **[Rupa Mobile](./facades/mobile.md)** | Pre-composed bundle for Native Mobile (Android/iOS). |
| **[Rupa Full-Stack](./facades/fullstack.md)** | The comprehensive all-in-one Rupa bundle. |
| **[Rupa Headless](./facades/headless.md)** | Pre-composed bundle for logic-only services. |
| **[Rupa Hybrid](./facades/hybrid.md)** | Bridge artisan showroom for JS/TS interoperability. |

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
| **[View Core](./architectures/view-core.md)** | Internal mechanism for VNode reconciliation. |
| **[VNode Separation](./architectures/logic-and-view.md)** | Modern conceptual guide for agnostic separation. |
| **[Module Standard](./architectures/module-standard.md)** | Crate-based workspace and naming conventions. |

### UI Primitives (Atomic)
| Module | Description |
| :--- | :--- |
| **[Primitive Design](./primitives/primitive-design.md)** | The design standard for atomic blocks (Primitives). |
| **[Overlay](./primitives/overlay.md)** | Absolute positioning and z-index management. |

### Semantic Components (Artisan)
| Module | Description |
| :--- | :--- |
| **[Component Design](./components/component-design.md)** | The design standard for semantic elements (Artisan). |
| **[Theme Control](./components/theme-switcher.md)** | Standardized Light/Dark mode switching logic. |

### App Compositions
| Module | Description |
| :--- | :--- |
| **[App Bootstrap](./compositions/app-bootstrap.md)** | Orchestration logic from App::new() to Runner::run(). |
| **[Control Flow](./compositions/control-flow.md)** | Logic components for conditional and list rendering. |
| **[Routing](./compositions/routing.md)** | Universal navigation and URL synchronization system. |

### Persistent Data System
| Module | Description |
| :--- | :--- |
| **[Persistent Overview](./ecosystems/persistent-data/overview.md)** | Core concepts of "Storage as a Signal". |
| **[Persistent Signals](./ecosystems/persistent-data/signal.md)** | Reactive wrapper for automatic data syncing. |
| **[Store Interface](./ecosystems/persistent-data/store.md)** | Agnostic contract for storage backends. |
| **[Storage Backends](./ecosystems/persistent-data/backends.md)** | Implementations for SQLite, WebStorage, and FS. |

### Network & Assets
| Module | Description |
| :--- | :--- |
| **[Network IO](./integrations/network-io.md)** | Reactive network requests and client config. |
| **[Network Resource](./integrations/network-io/resource.md)** | Async state container for fetching data. |
| **[Asset Management](./integrations/asset-management.md)** | Unified pipeline for fonts, images, and binary data. |
| **[Resource Loader](./integrations/asset-management/loader.md)** | Asynchronous fetching and decoding pipeline. |

### Motion & Animation
| Module | Description |
| :--- | :--- |
| **[Motion Engine](./ecosystems/motion-engine.md)** | High-performance VNode interpolation. |
| **[Spring Physics](./ecosystems/motion-engine/spring.md)** | Momentum-based reactive animations. |
| **[Transitions](./ecosystems/motion-engine/transition.md)** | Declarative property animations. |

### Accessibility & i18n
| Module | Description |
| :--- | :--- |
| **[A11y Bridge](./integrations/accessibility/bridge.md)** | Integration with OS accessibility services. |
| **[Node Translator](./integrations/accessibility/translate.md)** | Mapping VNodes to semantic OS roles. |
| **[i18n System](./ecosystems/i18n.md)** | Multi-language management and reactive translation. |
| **[Locale System](./ecosystems/i18n/locale.md)** | Regional settings and cultural formatting. |

### Identity & Auth
| Module | Description |
| :--- | :--- |
| **[Auth Overview](./ecosystems/auth.md)** | Reactive identity and session management. |
| **[User Identity](./ecosystems/auth/identity.md)** | Core data structures for authenticated users. |
| **[Session Lifecycle](./ecosystems/auth/session.md)** | Token management and auth state signals. |
| **[Access Control](./ecosystems/auth/rbac.md)** | Role-based permissions and guards. |
| **[Team Management](./ecosystems/auth/teams.md)** | Support for organizations and multi-tenancy. |

### Forms & Context
| Module | Description |
| :--- | :--- |
| **[Form Engine](./ecosystems/forms.md)** | High-level state management for UI inputs. |
| **[Reactive Validation](./ecosystems/forms/validation.md)** | Rules and logic for data verification. |
| **[Global Context](./ecosystems/context.md)** | Dependency injection for the VNode tree. |
| **[Context Provider](./ecosystems/context/provider.md)** | Injecting data into sub-trees. |

### Graphics & Telemetry
| Module | Description |
| :--- | :--- |
| **[Canvas System](./renderings/canvas.md)** | Low-level GPU drawing and meshes. |
| **[Graphics API](./renderings/canvas/api.md)** | Fluent interface for 2D/3D primitives. |
| **[Telemetry System](./integrations/telemetry.md)** | Observability and performance monitoring. |
| **[Performance Metrics](./integrations/telemetry/metrics.md)** | FPS, render times, and signal counts. |

### Developer Tooling
| Module | Description |
| :--- | :--- |
| **[Rupa CLI](./tooling/cli.md)** | Automated scaffolding and build pipelines. |
| **[CLI Commands](./tooling/cli/commands.md)** | Subcommands for the artisan workflow. |
| **[Console Reference](./console-references.md)** | Standardized TUI components for CLI output. |
| **[Testing Engine](./tooling/testing.md)** | QA utilities for headless verification. |
| **[Snapshot Testing](./tooling/testing/snapshot.md)** | Structural regression detection. |

### Ecosystems & Visual DNA
| Module | Description |
| :--- | :--- |
| **[Styling API](./ecosystems/styling-api.md)** | Functional "Utility-First" API reference. |
| **[Typography & Flow](./ecosystems/styling-typography.md)** | Styling reference for text and layout flow. |
| **[Theme Engine](./ecosystems/theme-engine.md)** | DNA Visual standard for consistent aesthetics. |
| **[Color Math](./ecosystems/color-math.md)** | OKLCH standards for perceptual uniformity. |
