# Rupa Framework Architectural Blueprint 🏛️

This document defines the structural integrity, dependency hierarchy, and execution flow of the **Rupa Framework**, a **modular meta-framework, cross-platform and multi-purpose**. It serves as the authoritative map for all engineering activities, ensuring compliance with the **ISO-IEC-12207-GEM-2026** governance.

---

## 1. Governance & Principles (The 3S Doctrine)

Every architectural decision in Rupa MUST be defensible under these three pillars:

*   **Secure (S1):** Protection of state integrity, strict boundary contracts, and deterministic failure semantics.
*   **Sustain (S2):** Semantic clarity, documentation parity, and reduced cognitive load through modularity.
*   **Scalable (S3):** Zero-cost abstractions, controlled dependency growth, and predictable performance under expansion.

---

## 2. Tiered Sub-System Architecture (The Macro View)

Rupa is organized into logical **Sub-Systems** that interact across three tiers based on the **[Artisan Workshop Standard](./architectures/workshop-tiers.md)** design pattern.

```mermaid
graph TD
    subgraph Tier_3 [Tier 3: Artisan Showrooms]
        facade[rupa Facade]
        desktop[rupa-desktop]
        web[rupa-web]
        server[rupa-server]
        tui[rupa-tui]
        mobile[rupa-mobile]
        fullstack[rupa-fullstack]
    end

    subgraph Tier_2 [Tier 2: Composite Assemblies]
        direction TB
        ui_sys[UI System - rupa-ui]
        eng_sys[Execution System - rupa-engine]
        srv_sys[Server Core - rupa-server-core]
        web_sys[Web Core - rupa-web-core]
        mob_sys[Mobile Core - rupa-mobile-core]
        tool_sys[Tooling System - rupa-cli / rupa-test]
    end

    subgraph Tier_1 [Tier 1: Atomic Materials]
        direction LR
        core_sys[Core System - rupa-core]
        react_sys[Reactive System - rupa-signals]
        vnode_sys[VNode & DNA - rupa-vnode]
        infra_sys[Infra: auth / store / net / i18n / assets / motion / context / forms / canvas]
    end

    %% Relationships
    facade --> Tier_2
    Tier_2 --> core_sys
    core_sys --> Tier_1
    
    style Tier_3 fill:#f9f,stroke:#333,stroke-width:2px
    style Tier_2 fill:#bbf,stroke:#333,stroke-width:2px
    style Tier_1 fill:#dfd,stroke:#333,stroke-width:2px
```

---

## 3. Sub-System Definitions & Responsibilities

### 3.1 Core & Reactive Systems (The Brain)
*   **Reactive System (`rupa-signals`)**: The "Nervous System". Handles **[Fine-Grained Updates](./reactivity/fine-grained-updates.md)** via `Signal` and `Memo`.
*   **VNode & DNA (`rupa-vnode`)**: The "Universal Language". Agnostic virtual tree structure and core style data models.
*   **Core System (`rupa-core`)**: The "Orchestrator". Manages component lifecycles and **[VNode Reconciliation](./architectures/reconciliation.md)**.

### 3.2 UI & Visual Systems (The Body)
*   **UI System (`rupa-ui`)**: Houses the **UI Component System** (Semantic elements) and **UI Utilities System** (Styling API).
*   **Motion Engine (`rupa-motion`)**: High-performance VNode interpolation and spring physics.
*   **Canvas System (`rupa-canvas`)**: Low-level hardware-accelerated drawing and custom shaders.

### 3.3 Platform & Execution Systems (The Muscles)
*   **Native Engines**: `rupa-engine` (GPU/TUI), `rupa-mobile-core`.
*   **Web Engines**: `rupa-server-core` (SSR), `rupa-web-core` (WASM).
*   **Tooling**: `rupa-cli` (DevOps), `rupa-test` (QA).

### 3.4 Enterprise Infrastructure Systems (The Foundation)
*   **Identity (`rupa-auth`)**: Reactive authentication, RBAC, and Team management.
*   **Persistence (`rupa-store`)**: "Storage as a Signal" bridge for SQLite, FS, and WebStorage.
*   **Connectivity**: `rupa-net` (Async I/O), `rupa-router` (Navigation), `rupa-i18n` (Voice).
*   **Management**: `rupa-assets` (Warehouse), `rupa-context` (DI), `rupa-telemetry` (Observability).

---

## 4. Internal Module Architecture (Detailed Mapping)

| Sub-System | Primary Modules | Key Exports |
| :--- | :--- | :--- |
| **Core** | `component`, `renderer`, `view`, `events` | `Component`, `Renderer`, `ViewCore` |
| **UI** | `elements`, `primitives`, `style` | `Button`, `Div`, `Theme` |
| **Signals** | `signal`, `memo`, `effect` | `Signal`, `Memo`, `Effect` |
| **VNode** | `vnode`, `style/*` | `VNode`, `Style`, `Color` |
| **Auth** | `identity`, `session`, `rbac`, `teams` | `User`, `Session`, `Role` |
| **Store** | `store`, `signal`, `backends` | `Store`, `PersistentSignal` |
| **Net** | `client`, `resource` | `Client`, `Resource`, `fetch` |
| **Motion** | `spring`, `transition`, `timeline` | `Spring`, `Transition` |
| **Router** | `router`, `route`, `history` | `Router`, `Route`, `use_route` |
| **i18n** | `provider`, `dictionary`, `locale` | `I18nProvider`, `t!`, `Locale` |
| **Assets** | `manager`, `loader`, `cache` | `AssetManager`, `use_asset` |
| **A11y** | `bridge`, `translate` | `A11yBridge` |
| **Context** | `provider`, `consumer` | `Provider`, `use_context` |
| **Telemetry**| `metrics`, `profiler`, `logger` | `Metrics`, `Profiler` |

---

## 5. Execution Pipeline (The Reactive Render Loop)

```mermaid
sequenceDiagram
    participant S as Reactive System (Signals)
    participant C as UI System (Components)
    participant V as VNode System (Virtual Tree)
    participant R as Core System (Reconciler)
    participant G as Execution System (WGPU/TUI/DOM)

    Note over S, G: Frame Lifecycle (Build -> Diff -> Patch)
    S->>C: Notify Change
    C->>C: render()
    C->>V: Produce VNode Snapshot
    V->>R: reconcile(old, new)
    R->>R: Identify Minimal Patches
    R->>G: render_patch(Patches)
    G->>G: Compute Layout (Taffy)
    G->>G: Draw Primitives (Batcher)
    G->>G: present()
```

---

## 6. Modular Pipeline Workflows (The Modular Choice)

Rupa Framework adapts its execution pipeline based on the target application. Below are the visual representations of how Atomic Materials and Composite Assemblies are assembled for different purposes.

### 6.1 Native Pipeline (Desktop & Mobile)
Focused on high-performance GPU/TUI rendering with direct hardware access.

```mermaid
graph LR
    subgraph Atoms [Atomic Materials]
        S[Signals]
        VN[VNode]
    end
    
    subgraph Composites [Composite Assemblies]
        UI[rupa-ui]
        Core[rupa-core]
        Eng[rupa-engine]
    end
    
    S --> UI
    UI --> VN
    VN --> Core
    Core -->|Patches| Eng
    Eng -->|WGPU/TUI| Hardware[GPU / Terminal]
```

### 6.2 Full-Stack Web Pipeline (SSR + Hydration)
Focused on SEO-friendly initial delivery and reactive client-side interactivity.

```mermaid
graph TD
    subgraph Server_Side [Server Core - rupa-server-core]
        S_UI[rupa-ui] -->|render| S_VN[VNode]
        S_VN -->|serialize| HTML[HTML String]
    end
    
    subgraph Client_Side [Web Core - rupa-web-core]
        C_UI[rupa-ui] -->|hydrate| C_VN[VNode]
        C_VN -->|diff/patch| DOM[Browser DOM]
    end
    
    HTML -.->|Delivery| Client_Side
```

---

## 7. Architectural Constraints & Standards

1.  **Strict Layering**: Atomic Materials (Tier 1) must never import from Composite Assemblies (Tier 2).
2.  **Agnostic Purity**: Foundational Atomic Materials must remain 100% free of OS-specific or hardware-specific code.
3.  **Serializability**: All data crossing system boundaries (VNodes, Styles, Events) MUST implement `serde`.
4.  **TDD Driven**: Every sub-system must be independently testable in a headless environment.
