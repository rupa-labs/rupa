# Crate References 📚

This is the official registry of all crates in the Rupa Framework monorepo, organized by their structural tier using our **Clean Naming Convention** (No Suffixes).

---

## 🧱 1. Atomic Materials (Low-Level Units)

Independent crates handling single responsibilities.

| Crate Name | Tier | Purpose |
| :--- | :--- | :--- |
| **`rupa-base`** | Atomic | Foundation utilities, math (Vec2), and common Errors. |
| **`rupa-signals`** | Atomic | The fine-grained reactivity engine. |
| **`rupa-vnode`** | Atomic | Universal UI DNA and Style data models. |
| **`rupa-motion`** | Atomic | Physics-based animation engine (Springs/Transitions). |
| **`rupa-console`** | Atomic | Low-level terminal infrastructure (ANSI, Raw Mode). |
| **`rupa-md`** | Atomic | Markdown & MDX parsing engine. |
| **`rupa-store`** | Atomic | Persistence system (SQLite, WebStorage). |
| **`rupa-net`** | Atomic | Reactive network I/O. |
| **`rupa-assets`** | Atomic | Binary resource pipeline (loading/caching). |
| **`rupa-auth`** | Atomic | Identity, RBAC, and session management. |
| **`rupa-i18n`** | Atomic | Internationalization and cultural formatting. |

---

## 🛠️ 2. Composite Assemblies (The Engines)

Logical systems that orchestrate materials into functional engines.

| Crate Name | Tier | Purpose |
| :--- | :--- | :--- |
| **`rupa-core`** | Composite | The heart: Reconciler, Layout, and Action Bus. |
| **`rupa-engine`** | Composite | The Agnostic Kernel: Lifecycle and Plugin orchestration. |
| **`rupa-ui`** | Composite | Agnostic semantic UI components (Buttons, Forms). |
| **`rupa-tui`** | Composite | Terminal-optimized UI components. |

---

## 🏪 3. Artisan Showrooms (The Platforms)

The "Physical Body" of the framework for specific hardware or targets.

| Crate Name | Tier | Target |
| :--- | :--- | :--- |
| **`rupa-desktop`** | Showroom | High-performance Native GUI (WGPU). |
| **`rupa-web`** | Showroom | Browser-based WASM & DOM rendering. |
| **`rupa-server`** | Showroom | Backend SSR & Static Site Generation. |
| **`rupa-mobile`** | Showroom | Native Android & iOS experiences. |
| **`rupa-terminal`** | Showroom | Interactive Terminal Applications. |
| **`rupa-cli`** | Showroom | Developer Tooling (Scaffolding, Build). |
| **`rupa`** | Showroom | The Universal Facade (All-in-one). |

---

## 🚀 Usage

Artisans typically depend on the `rupa` facade and enable features as needed:

```toml
[dependencies]
rupa = { git = "...", features = ["desktop", "web"] }
```
