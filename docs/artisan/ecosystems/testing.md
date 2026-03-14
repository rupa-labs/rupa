# Module: Testing & QA (`crates/rupa-test`) 🧪

The `rupa-test` crate provides specialized utilities for verifying Rupa applications in headless and interactive environments.

---

## 1. Core Philosophy: Agnostic Verification
Because Rupa is VNode-first, testing can occur without a physical window or GPU.

## 2. Module Structure (1:1 Mapping)
- `headless.rs`: Environment for rendering components to VNodes in memory.
- `snapshot.rs`: Utilities for comparing VNode trees against baseline files.
- `interaction.rs`: Simulation of user events (click, keypress) against the VNode tree.
