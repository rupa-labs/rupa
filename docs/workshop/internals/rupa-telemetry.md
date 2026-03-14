# `rupa-telemetry` 📊

**The Observability Port.** This crate provides **Atoms** for logging, metrics, and execution profiling. It is the "diagnostic hub" that makes Rupa applications transparent and maintainable.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Level-based filtering prevents sensitive debug information from reaching production logs.
    - **Sustain (S2)**: Unified `Telemetry` hub provides a consistent interface for all diagnostic needs.
    - **Scalable (S3)**: Pluggable adapters allow for high-performance metrics export (e.g., Prometheus, OpenTelemetry).

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Telemetry`** | Central Hub. | Orchestrates Logger, Recorder, and Profiler adapters. |
| **`Logger`** | The Log Port. | Supports Info, Warn, Error, and Debug levels. |
| **`Recorder`** | The Metrics Port. | Atomic Counters, Gauges, and Histogram Timings. |
| **`Profiler`** | The Trace Port. | Span-based execution tracking for performance hotspots. |
| **`MockTelemetry`**| Testing Hub. | Records all diagnostic data into in-memory buffers for tests. |

## 🚀 Usage

```rust
use rupa_telemetry::{Telemetry, Level, MockTelemetry};

// 1. Log a system event
Telemetry::global().logger.log(Level::Info, "engine", "Reconciliation started");

// 2. Record a performance metric
Telemetry::global().metrics.timing("frame_build_ms", 1.2);

// 3. Profile a critical section
Telemetry::global().profiler.start_span("diff_algorithm");
// ... compute ...
Telemetry::global().profiler.end_span("diff_algorithm");
```

## 🧪 Testing & Reliability
- **Log Verification**: `MockTelemetry` allows tests to assert that specific logs were emitted during an operation.
- **Metric Tracking**: Automated tests can verify performance bounds by inspecting recorded timings.
- **Zero-Overhead**: Null-adapters are provided by default to ensure zero performance cost when telemetry is not attached.
