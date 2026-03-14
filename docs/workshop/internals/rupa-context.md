# `rupa-context` 💉

**The Coordination Port.** This crate provides **Atoms** for dependency injection and scoped resource sharing. It is the "connective tissue" that allows decoupled components to access shared services.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: `thread_local` isolation prevents accidental context leakage between concurrent render passes.
    - **Sustain (S2)**: Hierarchical registry allows for clean overrides at any level of the component tree.
    - **Scalable (S3)**: Arc-based storage ensures that shared resources are never duplicated in memory.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`provide_context<T>`**| Inject data. | Injects a value into the current thread-local scope. |
| **`use_context<T>`** | Retrieve data. | Performs a hierarchical lookup for a registered type. |
| **`Registry`** | Scoped Container. | Thread-safe, hierarchical key-value map for types. |
| **`MockContext`** | Testing Helper. | Utility for running closures with a fresh, empty registry. |

## 🚀 Usage

```rust
use rupa_context::{provide_context, use_context, MockContext};

// 1. Provide a service at the app root
provide_context(Theme::Dark);

// 2. Consume in a deeply nested component (Atom or Composite)
let theme = use_context::<Theme>().unwrap_or_default();

// 3. Scoped override
let custom_scope = create_scoped_registry();
with_registry(custom_scope, || {
    provide_context(Theme::Light);
    // Inside here, use_context returns Light
});
```

## 🧪 Testing & Reliability
- **Isolation**: `MockContext::run` ensures that tests do not pollute each other's registries.
- **Hierarchy Verified**: Automated tests confirm that child registries correctly fall back to parent values.
- **TDD Driven**: The entire DI system is verified for zero-memory-leak performance.
