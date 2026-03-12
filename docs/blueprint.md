# Rupa Framework Architectural Blueprint 🏛️

This document defines the structural integrity, dependency hierarchy, and execution flow of the **Rupa Framework**, a **modular meta-framework, cross-platform and multi-purpose**. It serves as the authoritative map for all engineering activities, ensuring compliance with the **ISO-IEC-12207-GEM-2026** governance.

---

## 1. Governance & Principles (The 3S Doctrine)

Every architectural decision in Rupa MUST be defensible under these three pillars:

*   **Secure (S1):** Protection of state integrity, strict boundary contracts, and deterministic failure semantics.
*   **Sustain (S2):** Semantic clarity, documentation parity, and reduced cognitive load through modularity.
*   **Scalable (S3):** Zero-cost abstractions, controlled dependency growth, and predictable performance under expansion.

---

## 2. Tiered Hexagonal Architecture (The Macro View)

Rupa Framework is organized into three layers of responsibility, following the **Ports and Adapters** model to achieve *zero-coupling* between core logic and infrastructure.

```mermaid
graph TD
    subgraph Tier_3 [Tier 3: Artisan Showrooms - THE ADAPTERS]
        direction TB
        rupa[rupa / Universal Facade]
        desktop[rupa-desktop / WGPU]
        web[rupa-web / WASM]
        terminal[rupa-terminal / Crossterm]
        mobile[rupa-mobile / Native]
        server[rupa-server / SSR]
        fullstack[rupa-fullstack / Hybrid]
        cli[rupa-cli / Tooling]
    end

    subgraph Tier_2 [Tier 2: Composite Assemblies - THE CORE]
        direction TB
        kernel[rupa-engine / Agnostic Kernel]
        core[rupa-core / Reconciler]
        ui[rupa-ui / Agnostic Components]
        md[rupa-md / Markdown Engine]
    end

    subgraph Tier_1 [Tier 1: Atomic Materials - THE PORTS & DNA]
        direction LR
        signals[rupa-signals / DNA]
        vnode[rupa-vnode / DNA]
        ports[Infra Ports: auth / store / net / i18n / broadcast / queue / telemetry / a11y]
        base[rupa-base / Standard Types]
    end

    %% Relationships
    Tier_3 -->|Implements| Tier_1
    Tier_2 -->|Orchestrates| Tier_1
    Tier_3 -->|Consumes| Tier_2
    
    style Tier_3 fill:#f9f,stroke:#333,stroke-width:2px
    style Tier_2 fill:#bbf,stroke:#333,stroke-width:2px
    style Tier_1 fill:#dfd,stroke:#333,stroke-width:2px
```

---

## 3. Sub-System Definitions & Responsibilities

### 3.1 The DNA & Ports (Tier 1)
*   **The DNA**: `rupa-signals` (Fine-grained reactivity) and `rupa-vnode` (Universal UI language).
*   **The Ports**: Foundational traits that define *what* the system can do.
    *   `auth::Service`, `store::Store`, `net::Client`, `broadcast::Broadcaster`.
*   **Standard Materials**: `rupa-base` (Types), `rupa-motion` (Animation), `rupa-test` (TDD Support).

### 3.2 The Core Kernel (Tier 2)
*   **The Brain**: `rupa-core`. Manages the virtual tree, diffing algorithm, and action dispatching.
*   **The Orchestrator**: `rupa-engine`. Manages the universal application lifecycle (`App`).
*   **The Toolkit**: `rupa-ui` (Platform-agnostic semantic components) and `rupa-md` (Content engine).

### 3.3 The Showroom Adapters (Tier 3)
*   **Platform Targets**: 
    *   `rupa-desktop` (GPU), `rupa-terminal` (ANSI), `rupa-web` (Browser).
    *   `rupa-mobile` (Android/iOS), `rupa-server` (SSR/API).
*   **Hybrid Solutions**: `rupa-fullstack` manages hydration between server and client.
*   **Artisan Tools**: `rupa-cli` for scaffolding and developer experience.
*   **Universal Facade**: The `rupa` crate provides a unified entry point for all features.

---

## 4. Internal Module Architecture (Detailed Mapping)

| Sub-System | Primary Modules | Key Exports (Ergonomic) |
| :--- | :--- | :--- |
| **Core** | `reconciler`, `renderer`, `view`, `events` | `Core`, `Renderer`, `Patch` |
| **UI** | `elements`, `primitives`, `style` | `Button`, `Div`, `Theme` |
| **Signals** | `signal`, `memo`, `effect` | `Signal`, `Memo`, `Effect` |
| **VNode** | `vnode`, `style/*` | `VNode`, `Style`, `Color` |
| **Auth** | `identity`, `session`, `rbac`, `teams` | `User`, `Status`, `Service`, `Port` |
| **Store** | `store`, `signal`, `backends` | `Store`, `PersistentSignal`, `Cache` |
| **Net** | `client`, `resource` | `Client`, `Resource`, `Fetch` |
| **Motion** | `spring`, `transition`, `timeline` | `Spring`, `Transition`, `Easing` |
| **i18n** | `provider`, `dictionary`, `locale` | `Provider`, `Translator`, `translate` |
| **Queue** | `task`, `queue` | `Task`, `Queue`, `Handle` |
| **Forms** | `validation`, `rules`, `form` | `Form`, `Field`, `Validator` |
| **A11y** | `bridge`, `node`, `translate` | `Bridge`, `Node`, `Manager` |
| **Context** | `registry`, `provider` | `Registry`, `use_context` |
| **Telemetry**| `logger`, `metrics`, `profiler` | `Telemetry`, `Logger`, `Recorder` |
| **Test** | `headless`, `snapshot` | `Tester`, `Snapshot`, `setup` |

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
